use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pid(pub(crate) u32);

impl From<u32> for Pid {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
