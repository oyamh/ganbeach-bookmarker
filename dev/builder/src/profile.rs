use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Profile {
    Dev,
    Release,
}

impl Profile {
    pub fn is_release(&self) -> bool {
        match self {
            Self::Release => true,
            _ => false,
        }
    }
}

impl From<bool> for Profile {
    fn from(src: bool) -> Self {
        match src {
            false => Self::Dev,
            true => Self::Release,
        }
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dev => write!(f, "{}", "debug"),
            Self::Release => write!(f, "{}", "release"),
        }
    }
}
