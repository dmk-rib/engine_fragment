#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigShellProfile {
    pub indices: Vec<u32>,
}

impl BigShellProfile {
    pub fn new(indices: Vec<u32>) -> Self {
        Self { indices }
    }
}

impl Default for BigShellProfile {
    fn default() -> Self {
        Self {
            indices: Vec::new(),
        }
    }
}
