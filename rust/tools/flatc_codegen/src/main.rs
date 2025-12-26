use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let mut schema_path: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut flatc_path: Option<PathBuf> = None;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--schema" => {
                if let Some(value) = args.next() {
                    schema_path = Some(PathBuf::from(value));
                }
            }
            "--out-dir" => {
                if let Some(value) = args.next() {
                    out_dir = Some(PathBuf::from(value));
                }
            }
            "--flatc" => {
                if let Some(value) = args.next() {
                    flatc_path = Some(PathBuf::from(value));
                }
            }
            _ => {}
        }
    }

    let schema_path = match schema_path {
        Some(path) => path,
        None => {
            eprintln!("flatc_codegen: missing --schema <path-to-index.fbs>");
            return ExitCode::from(2);
        }
    };

    let out_dir = match out_dir {
        Some(path) => path,
        None => {
            eprintln!("flatc_codegen: missing --out-dir <output-directory>");
            return ExitCode::from(2);
        }
    };

    let flatc = match flatc_path {
        Some(path) => path,
        None => match which::which("flatc") {
            Ok(path) => path,
            Err(_) => match download_flatc() {
                Ok(path) => path,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(3);
                }
            },
        },
    };

    let status = Command::new(&flatc)
        .arg("--rust")
        .arg("--gen-object-api")
        .arg("-o")
        .arg(&out_dir)
        .arg(&schema_path)
        .status();

    match status {
        Ok(result) if result.success() => {
            if let Err(error) = post_process_generated(&out_dir) {
                eprintln!("flatc_codegen: {error}");
                return ExitCode::from(5);
            }
            ExitCode::SUCCESS
        }
        Ok(result) => {
            eprintln!("flatc_codegen: flatc failed with status {result}");
            ExitCode::from(4)
        }
        Err(error) => {
            eprintln!("flatc_codegen: failed to invoke flatc: {error}");
            ExitCode::from(4)
        }
    }
}

fn download_flatc() -> Result<PathBuf, String> {
    const VERSION: &str = "23.5.26";
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bin_dir = manifest_dir.join("bin");
    fs::create_dir_all(&bin_dir).map_err(|error| {
        format!("flatc_codegen: failed to create bin directory {bin_dir:?}: {error}")
    })?;
    let flatc_path = bin_dir.join("flatc");
    if flatc_path.exists() {
        return Ok(flatc_path);
    }

    let os = env::consts::OS;
    let url = match os {
        "linux" => format!(
            "https://github.com/google/flatbuffers/releases/download/v{VERSION}/Linux.flatc.binary"
        ),
        "macos" => format!(
            "https://github.com/google/flatbuffers/releases/download/v{VERSION}/Mac.flatc.binary"
        ),
        _ => {
            return Err(format!(
                "flatc_codegen: unsupported OS '{os}'. Install flatc manually and rerun."
            ))
        }
    };

    let status = Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(&flatc_path)
        .arg(&url)
        .status()
        .map_err(|error| format!("flatc_codegen: failed to invoke curl: {error}"))?;
    if !status.success() {
        return Err(format!(
            "flatc_codegen: curl failed downloading flatc from {url} (status {status})"
        ));
    }

    let status = Command::new("chmod")
        .arg("+x")
        .arg(&flatc_path)
        .status()
        .map_err(|error| format!("flatc_codegen: failed to chmod flatc: {error}"))?;
    if !status.success() {
        return Err(format!(
            "flatc_codegen: chmod failed for {flatc_path:?} (status {status})"
        ));
    }

    Ok(flatc_path)
}

fn post_process_generated(out_dir: &Path) -> Result<(), String> {
    let mut modules = Vec::new();
    for entry in fs::read_dir(out_dir)
        .map_err(|error| format!("failed to read generated dir {out_dir:?}: {error}"))?
    {
        let entry = entry.map_err(|error| format!("failed to read entry: {error}"))?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        if path.file_name().and_then(|name| name.to_str()) == Some("mod.rs") {
            continue;
        }
        modules.push(path);
    }

    modules.sort();
    for module_path in &modules {
        prepend_header(module_path)?;
    }

    let mod_path = out_dir.join("mod.rs");
    let mut contents = String::from("// DO NOT EDIT: generated by flatc_codegen\n\n");
    for module_path in &modules {
        let stem = module_path
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("invalid module filename: {module_path:?}"))?;
        contents.push_str(&format!("pub mod {stem};\n"));
    }
    fs::write(&mod_path, contents)
        .map_err(|error| format!("failed to write {mod_path:?}: {error}"))?;

    Ok(())
}

fn prepend_header(path: &Path) -> Result<(), String> {
    let header = "// DO NOT EDIT: generated by flatc_codegen\n\n";
    let contents =
        fs::read_to_string(path).map_err(|error| format!("failed to read {path:?}: {error}"))?;
    if contents.starts_with(header) {
        return Ok(());
    }
    fs::write(path, format!("{header}{contents}"))
        .map_err(|error| format!("failed to write {path:?}: {error}"))?;
    Ok(())
}

mod which {
    use std::env;
    use std::path::{Path, PathBuf};

    pub fn which(binary: &str) -> Result<PathBuf, ()> {
        let path = match env::var_os("PATH") {
            Some(path) => path,
            None => return Err(()),
        };
        for entry in env::split_paths(&path) {
            let candidate = entry.join(binary);
            if is_executable(&candidate) {
                return Ok(candidate);
            }
        }
        Err(())
    }

    fn is_executable(path: &Path) -> bool {
        path.is_file()
    }
}
