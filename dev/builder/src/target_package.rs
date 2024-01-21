use crate::error::BuildError;
use std::{str::FromStr, vec::IntoIter};

pub const PACKAGE_ALL: &'static str = "all";
pub const PACKAGE_BACKGROUND: &'static str = "background";
// pub const PACKAGE_CONTENT: &'static str = "content";
pub const PACKAGE_POPUP: &'static str = "popup";
pub const PACKAGE_STATIC: &'static str = "static";

#[derive(Debug, Clone, Copy)]
pub enum TargetPackage {
    All,
    Background,
    // Content,
    Popup,
    Static,
}

impl Into<&str> for &TargetPackage {
    fn into(self) -> &'static str {
        match self {
            TargetPackage::All => PACKAGE_ALL,
            TargetPackage::Background => PACKAGE_BACKGROUND,
            // TargetPackage::Content => PACKAGE_CONTENT,
            TargetPackage::Popup => PACKAGE_POPUP,
            TargetPackage::Static => PACKAGE_STATIC,
        }
    }
}

impl Into<&str> for TargetPackage {
    fn into(self) -> &'static str {
        Into::into(&self)
    }
}

impl TryFrom<&str> for TargetPackage {
    type Error = BuildError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            PACKAGE_ALL => Ok(TargetPackage::All),
            PACKAGE_BACKGROUND => Ok(TargetPackage::Background),
            // PACKAGE_CONTENT => Ok(TargetPackage::Content),
            PACKAGE_POPUP => Ok(TargetPackage::Popup),
            PACKAGE_STATIC => Ok(TargetPackage::Static),
            _ => Err(BuildError::UnknownPackage),
        }
    }
}

impl Into<String> for TargetPackage {
    fn into(self) -> String {
        Into::<&str>::into(self).to_string()
    }
}

impl std::fmt::Display for TargetPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}

impl FromStr for TargetPackage {
    type Err = BuildError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

#[test]
fn should_display() {
    let p = TargetPackage::Background;
    println!("{}", p);
    assert_eq!(PACKAGE_BACKGROUND, p.to_string());
}

impl Into<Vec<TargetPackage>> for TargetPackage {
    fn into(self) -> Vec<TargetPackage> {
        match self {
            Self::All => vec![Self::Background, Self::Popup], //Self::Content,
            Self::Background => vec![Self::Background],
            // Self::Content => vec![Self::Content],
            Self::Popup => vec![Self::Popup],
            Self::Static => vec![],
        }
    }
}

impl IntoIterator for TargetPackage {
    type Item = TargetPackage;
    type IntoIter = IntoIter<TargetPackage>;
    fn into_iter(self) -> Self::IntoIter {
        Into::<Vec<Self::Item>>::into(self).into_iter()
    }
}
