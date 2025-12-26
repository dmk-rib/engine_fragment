#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AxisPartClass {
    None = 0,
    Wire = 1,
    WireSet = 2,
    CircleCurve = 3,
}

impl Default for AxisPartClass {
    fn default() -> Self {
        Self::None
    }
}
