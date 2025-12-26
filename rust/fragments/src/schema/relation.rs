#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relation {
    pub data: Vec<String>,
}

impl Relation {
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }
}

impl Default for Relation {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}
