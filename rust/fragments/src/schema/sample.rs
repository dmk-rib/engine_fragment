#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sample {
    pub item: u32,
    pub material: u32,
    pub representation: u32,
    pub local_transform: u32,
}

impl Sample {
    pub fn new(item: u32, material: u32, representation: u32, local_transform: u32) -> Self {
        Self {
            item,
            material,
            representation,
            local_transform,
        }
    }
}

impl Default for Sample {
    fn default() -> Self {
        Self {
            item: 0,
            material: 0,
            representation: 0,
            local_transform: 0,
        }
    }
}
