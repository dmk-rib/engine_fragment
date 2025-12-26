#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RepresentationClass {
    None = 0,
    Shell = 1,
    CircleExtrusion = 2,
}

impl Default for RepresentationClass {
    fn default() -> Self {
        Self::None
    }
}
