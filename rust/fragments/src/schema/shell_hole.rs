#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellHole {
    pub indices: Vec<u16>,
    pub profile_id: u16,
}

impl ShellHole {
    pub fn new(indices: Vec<u16>, profile_id: u16) -> Self {
        Self {
            indices,
            profile_id,
        }
    }
}

impl Default for ShellHole {
    fn default() -> Self {
        Self {
            indices: Vec::new(),
            profile_id: 0,
        }
    }
}
