use crate::schema::{
    BigShellHole, BigShellProfile, FloatVector, ShellHole, ShellProfile, ShellType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Shell {
    pub profiles: Vec<ShellProfile>,
    pub holes: Vec<ShellHole>,
    pub points: Vec<FloatVector>,
    pub big_profiles: Vec<BigShellProfile>,
    pub big_holes: Vec<BigShellHole>,
    pub shell_type: ShellType,
    pub profiles_face_ids: Vec<u16>,
}

impl Shell {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        profiles: Vec<ShellProfile>,
        holes: Vec<ShellHole>,
        points: Vec<FloatVector>,
        big_profiles: Vec<BigShellProfile>,
        big_holes: Vec<BigShellHole>,
        shell_type: ShellType,
        profiles_face_ids: Vec<u16>,
    ) -> Self {
        Self {
            profiles,
            holes,
            points,
            big_profiles,
            big_holes,
            shell_type,
            profiles_face_ids,
        }
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            profiles: Vec::new(),
            holes: Vec::new(),
            points: Vec::new(),
            big_profiles: Vec::new(),
            big_holes: Vec::new(),
            shell_type: ShellType::default(),
            profiles_face_ids: Vec::new(),
        }
    }
}
