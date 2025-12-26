#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Stroke {
    Default = 0,
}

impl Default for Stroke {
    fn default() -> Self {
        Self::Default
    }
}
