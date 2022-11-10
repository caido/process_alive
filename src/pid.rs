use std::fmt;
use std::num::TryFromIntError;

/// Used as a wrapper for the process ID
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pid(pub(crate) u32);

impl From<u32> for Pid {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl TryFrom<i32> for Pid {
    type Error = TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
