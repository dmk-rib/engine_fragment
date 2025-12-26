#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub data: Vec<String>,
}

impl Attribute {
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }
}

impl Default for Attribute {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}
