use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Alive,
    Dead,
    Unknown,
}

impl State {
    pub fn is_alive(&self) -> bool {
        *self == Self::Alive
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Alive => "alive",
            Self::Dead => "dead",
            Self::Unknown => "unknown",
        };
        write!(f, "{}", s)
    }
}
