#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ShellType {
    None = 0,
    Big = 1,
}

impl Default for ShellType {
    fn default() -> Self {
        Self::None
    }
}
