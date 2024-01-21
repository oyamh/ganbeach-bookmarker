use std::{fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::{BookmarkId, DomainError};

#[derive(Debug, Eq, Hash, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Title(String);

impl Title {
    pub fn new<T>(name: T) -> Self
    where
        T: AsRef<str>,
    {
        Self(name.as_ref().to_string())
    }
}

impl AsRef<str> for Title {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for Title {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl Into<String> for Title {
    fn into(self) -> String {
        self.0
    }
}

impl From<&str> for Title {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl Deref for Title {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// URLのパースは行わない。自由度を重視。URLの他にbookmarkletも許可するため。
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PageUrl(String);

impl PageUrl {
    pub fn new<T>(url: T) -> Self
    where
        T: AsRef<str>,
    {
        Self(url.as_ref().to_string())
    }
}

impl AsRef<str> for PageUrl {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for PageUrl {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl Into<String> for PageUrl {
    fn into(self) -> String {
        self.0
    }
}

impl From<&str> for PageUrl {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

// #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub struct BookmarkId(u64);

// impl BookmarkId {
//     pub fn new(id: u64) -> Self {
//         Self(id)
//     }
// }

// impl From<u64> for BookmarkId {
//     fn from(id: u64) -> Self {
//         Self(id)
//     }
// }

// impl From<BookmarkId> for u64 {
//     fn from(bookmark_id: BookmarkId) -> Self {
//         bookmark_id.0
//     }
// }

// impl Display for BookmarkId {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Titles(Vec<Title>);

impl Titles {
    pub fn new(titles: Vec<Title>) -> Self {
        Self(titles)
    }
}

impl From<Vec<Title>> for Titles {
    fn from(value: Vec<Title>) -> Self {
        Self::new(value)
    }
}

impl Into<Vec<Title>> for Titles {
    fn into(self) -> Vec<Title> {
        self.0
    }
}

impl Into<Vec<String>> for Titles {
    fn into(self) -> Vec<String> {
        self.0.iter().map(|title| title.to_string()).collect()
    }
}

impl From<Vec<&str>> for Titles {
    fn from(str_vec: Vec<&str>) -> Self {
        Self(str_vec.into_iter().map(Into::into).collect::<Vec<Title>>())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagIds(Vec<BookmarkId>);

impl TagIds {
    pub fn new(ids: Vec<BookmarkId>) -> Self {
        Self(ids)
    }

    pub fn to_vec(&self) -> Self {
        Self(self.0.to_vec())
    }

    pub fn iter(&self) -> impl Iterator<Item = &BookmarkId> {
        self.0.iter()
    }
}

impl Display for TagIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        let ids: Vec<String> = self.0.iter().map(|id| id.to_string()).collect();
        f.write_str(&ids.join(","))?;
        f.write_str("]")?;
        Ok(())
    }
}

impl From<Vec<u64>> for TagIds {
    fn from(value: Vec<u64>) -> Self {
        Self::new(value.into_iter().map(|id| id.into()).collect())
    }
}

impl Into<Vec<u64>> for TagIds {
    fn into(self) -> Vec<u64> {
        self.0
            .into_iter()
            .map(|bookmark_id| bookmark_id.into())
            .collect()
    }
}

impl From<Vec<BookmarkId>> for TagIds {
    fn from(value: Vec<BookmarkId>) -> Self {
        Self::new(value)
    }
}

impl Into<Vec<BookmarkId>> for TagIds {
    fn into(self) -> Vec<BookmarkId> {
        self.0
    }
}

// impl Into<Vec<u64>> for TagIds {
//     fn into(self) -> Vec<BookmarkId> {
//         self.0
//     }
// }

impl Into<Vec<String>> for TagIds {
    fn into(self) -> Vec<String> {
        self.0.iter().map(|id| id.to_string()).collect()
    }
}

impl Into<String> for TagIds {
    fn into(self) -> String {
        Into::<Vec<String>>::into(self).join(",")
    }
}

// impl From<String> for TagIds {
//     fn from(value: String) -> Self {
//         value
//             .split(",")
//             .map(|str_id| BookmarkId::try_from(str_id).unwrap())
//             .collect::<Vec<BookmarkId>>()
//             .into()
//     }
// }

impl TryFrom<String> for TagIds {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(value
            .split(",")
            .map(|str_id| BookmarkId::try_from(str_id).unwrap_or_default())
            .collect::<Vec<BookmarkId>>()
            .into())
    }
}

// impl IntoIterator for TagIds {
//     type Item = BookmarkId;
//     type IntoIter = IntoIter<Self::Item>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// impl Iterator for TagIds {
//     type Item = BookmarkId;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.iter()
//     }
// }

// struct TagIdsEnumerate<T> {
//     iter: I,
//     length: u32,
//     index: u32,
// }

// impl<I> Iterator for TagIdsEnumerate<I> where T: Iterator {
//     type Item = I::Item;
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.iter.next() {
//             Some(value) => {
//                 let index = self.index;
//                 self.index = (self.index + 1) % self.items;
//                 Some(value)
//             }
//             None => None,
//         }
//     }
// }

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Annotation(String);

impl Annotation {
    pub fn new<T>(annotation: T) -> Self
    where
        T: AsRef<str>,
    {
        Self(annotation.as_ref().to_string())
    }
}

impl From<String> for Annotation {
    fn from(src: String) -> Self {
        Self::new(src)
    }
}

impl Into<String> for Annotation {
    fn into(self) -> String {
        self.0
    }
}

impl From<&str> for Annotation {
    fn from(src: &str) -> Self {
        Self::new(src)
    }
}
