#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellProfile {
    pub indices: Vec<u16>,
}

impl ShellProfile {
    pub fn new(indices: Vec<u16>) -> Self {
        Self { indices }
    }
}

impl Default for ShellProfile {
    fn default() -> Self {
        Self {
            indices: Vec::new(),
        }
    }
}
