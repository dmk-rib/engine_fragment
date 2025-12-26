#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RenderedFaces {
    One = 0,
    Two = 1,
}

impl Default for RenderedFaces {
    fn default() -> Self {
        Self::One
    }
}
