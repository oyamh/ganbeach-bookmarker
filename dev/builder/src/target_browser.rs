use crate::error::BuildError;
use std::{str::FromStr, vec::IntoIter};

pub const BROWSER_ALL: &'static str = "all";
pub const BROWSER_CHROME: &'static str = "chrome";
pub const BROWSER_FIREFOX: &'static str = "firefox";

#[derive(Debug, Clone, Copy)]
pub enum TargetBrowser {
    All,
    Chrome,
    Firefox,
}

#[derive(Debug, Copy, Clone)]
enum ManifestVersion {
    V2 = 2,
    V3 = 3,
}

impl TargetBrowser {
    pub fn manifest_version(&self) -> u8 {
        match self {
            TargetBrowser::All => 0,
            TargetBrowser::Chrome => ManifestVersion::V3 as u8,
            TargetBrowser::Firefox => ManifestVersion::V2 as u8,
        }
    }

    pub fn unused_manifest_version(&self) -> u8 {
        match self {
            TargetBrowser::All => 0,
            TargetBrowser::Chrome => ManifestVersion::V2 as u8,
            TargetBrowser::Firefox => ManifestVersion::V3 as u8,
        }
    }
}

impl Into<&str> for &TargetBrowser {
    fn into(self) -> &'static str {
        match self {
            TargetBrowser::All => BROWSER_ALL,
            TargetBrowser::Chrome => BROWSER_CHROME,
            TargetBrowser::Firefox => BROWSER_FIREFOX,
        }
    }
}

impl Into<&str> for TargetBrowser {
    fn into(self) -> &'static str {
        Into::into(&self)
    }
}

impl TryFrom<&str> for TargetBrowser {
    type Error = BuildError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            BROWSER_ALL => Ok(TargetBrowser::All),
            BROWSER_CHROME => Ok(TargetBrowser::Chrome),
            BROWSER_FIREFOX => Ok(TargetBrowser::Firefox),
            _ => Err(BuildError::UnknownBrowser),
        }
    }
}

impl Into<String> for TargetBrowser {
    fn into(self) -> String {
        Into::<&str>::into(self).to_string()
    }
}

impl TryFrom<String> for TargetBrowser {
    type Error = BuildError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.try_into()
        // match &*value {
        //     "chrome" => Ok(TargetBrowser::Chrome),
        //     "firefox" => Ok(TargetBrowser::Firefox),
        //     _ => Err(BuildError::UnknownTarget),
        // }
    }
}

impl std::fmt::Display for TargetBrowser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
        // write!(f, "{:?}", self)
    }
}

impl FromStr for TargetBrowser {
    type Err = BuildError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl Into<Vec<TargetBrowser>> for TargetBrowser {
    fn into(self) -> Vec<TargetBrowser> {
        match self {
            TargetBrowser::All => vec![Self::Chrome, Self::Firefox],
            TargetBrowser::Chrome => vec![Self::Chrome],
            TargetBrowser::Firefox => vec![Self::Firefox],
        }
    }
}

impl IntoIterator for TargetBrowser {
    type Item = TargetBrowser;
    type IntoIter = IntoIter<TargetBrowser>;
    fn into_iter(self) -> Self::IntoIter {
        Into::<Vec<Self::Item>>::into(self).into_iter()
    }
}
