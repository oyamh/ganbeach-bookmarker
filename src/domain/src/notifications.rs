use serde::{Deserialize, Serialize};
use url::Url;

use crate::DomainError;
use std::str::FromStr;

#[derive(Debug)]
pub enum NotificationEvent {
    // OnButtonClicked,
    OnClicked,
    OnClosed,
    // OnShown, // firefox only
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct NotificationId(String);

// impl NotificationId {
//     pub fn new(id: impl AsRef<str>) -> Self {
//         Self(id.as_ref().to_string())
//     }
// }

// impl Deref for NotificationId {
//     type Target = String;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationId {
    String(String),
    Url(Url),
}

impl NotificationId {
    pub fn from_url(url_str: impl AsRef<str>) -> Result<Self, DomainError> {
        Ok(Self::Url(
            Url::parse(url_str.as_ref()).map_err(|error| DomainError::ParseUrl(error))?,
        ))
    }

    pub fn parse(id: impl AsRef<str>) -> Self {
        let id = id.as_ref();
        match id {
            id if id.starts_with(NOTIFICATION_URL_PREFIX) => {
                let Some(url_str) = id.strip_prefix(NOTIFICATION_URL_PREFIX) else {
                    return Self::String(id.to_string());
                };
                let Ok(url) = Url::parse(url_str) else {
                    return Self::String(id.to_string());
                };
                Self::Url(url)
            }
            _ => Self::String(id.to_string()),
        }
    }
}

impl From<&str> for NotificationId {
    fn from(id_str: &str) -> Self {
        id_str.to_string().into()
    }
}

impl From<String> for NotificationId {
    fn from(id_str: String) -> Self {
        Self::String(id_str)
    }
}

impl From<Url> for NotificationId {
    fn from(url: Url) -> Self {
        Self::Url(url)
    }
}

impl TryInto<Url> for NotificationId {
    type Error = DomainError;
    fn try_into(self) -> Result<Url, Self::Error> {
        match self {
            Self::Url(url) => Ok(url),
            Self::String(id_str) => Err(Self::Error::InvalidNotificationId(id_str)),
        }
    }
}

const NOTIFICATION_URL_PREFIX: &'static str = "Url__";

impl Into<String> for NotificationId {
    fn into(self) -> String {
        use NotificationId::*;
        match self {
            String(string_id) => string_id,
            Url(url_id) => format!("{}{}", NOTIFICATION_URL_PREFIX, url_id),
        }
    }
}

// pub struct NotificationUrl(Url);

// impl NotificationUrl {
//     pub fn parse(id: impl AsRef<str>) -> Result<Self, DomainError> {
//         Ok(Self(Url::parse(id.as_ref()).map_err(|error| {
//             DomainError::Parse(format!("parse Url error: {error}"))
//         })?))
//     }
// }

#[derive(Debug, Default)]
pub enum TemplateType {
    #[default]
    Basic,
    Image,    // Basic + .imageUrl
    List,     // Basic + .items
    Progress, // Basic + .progress
}

impl Into<&'static str> for &TemplateType {
    fn into(self) -> &'static str {
        use TemplateType::*;
        match self {
            Basic => "basic",
            Image => "image",
            List => "list",
            Progress => "progress",
        }
    }
}

impl Serialize for TemplateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

/// https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/notifications/NotificationOptions
#[derive(Debug, Serialize)]
pub struct NotificationOptions {
    #[serde(rename = "type")]
    template_type: TemplateType,
    message: String,
    title: String,
    #[serde(rename = "iconUrl")]
    icon_url: String,
    // icon_url: Path,
    // firefox no support.
    // context_message: String,
    // priority: u8, //-2 to 2. 0 is default.
}

impl NotificationOptions {
    pub fn new(message: impl AsRef<str>, title: impl AsRef<str>) -> Self {
        Self {
            template_type: TemplateType::Basic,
            message: message.as_ref().to_string(),
            title: title.as_ref().to_string(),
            icon_url: String::from_str("../icons/book48.png").unwrap(),
            // icon_url: Path::new("../icons/book48.png"),
        }
    }
}
