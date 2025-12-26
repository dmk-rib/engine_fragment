#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialStructure {
    pub local_id: Option<u32>,
    pub category: Option<String>,
    pub children: Vec<SpatialStructure>,
}

impl SpatialStructure {
    pub fn new(
        local_id: Option<u32>,
        category: Option<String>,
        children: Vec<SpatialStructure>,
    ) -> Self {
        Self {
            local_id,
            category,
            children,
        }
    }
}

impl Default for SpatialStructure {
    fn default() -> Self {
        Self {
            local_id: None,
            category: None,
            children: Vec::new(),
        }
    }
}
