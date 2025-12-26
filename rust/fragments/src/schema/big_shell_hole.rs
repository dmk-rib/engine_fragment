#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigShellHole {
    pub indices: Vec<u32>,
    pub profile_id: u16,
}

impl BigShellHole {
    pub fn new(indices: Vec<u32>, profile_id: u16) -> Self {
        Self {
            indices,
            profile_id,
        }
    }
}

impl Default for BigShellHole {
    fn default() -> Self {
        Self {
            indices: Vec::new(),
            profile_id: 0,
        }
    }
}
