use std::{fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::JsValue;

use crate::{DomainError, Title};

// pub type UserId = u64;
// pub type BookmarkId = u64;
// pub type ParentId = u64;
// pub type TypeCode = u32;
type ChildCount = u32;
type Position = u64;
type ListTime = i64;
// pub type ListTitle = String;
// type TagNames = Vec<TagName>;
// pub type TagName = String;

type IdBytes = [u8; 8];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BookmarkId(pub u64);

type UserId = BookmarkId;
type ParentId = BookmarkId;

impl BookmarkId {
    pub fn as_bytes(&self) -> IdBytes {
        Into::<IdBytes>::into(*self)
    }
}

impl Deref for BookmarkId {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for BookmarkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Into<u64> for BookmarkId {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for BookmarkId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl TryFrom<&str> for BookmarkId {
    type Error = DomainError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = u64::from_str_radix(value, 10);
        match result {
            Ok(id) => Ok(Self(id)),
            Err(error) => Err(DomainError::ParseBookmarkId(error.to_string())),
        }
    }
}

impl Into<String> for BookmarkId {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<String> for BookmarkId {
    type Error = DomainError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
        // let result = u64::from_str_radix(&value, 10);
        // match result {
        //     Ok(id) => Ok(Self(id)),
        //     Err(error) => Err(DomainError::Parse(format!("parse error: {error}"))),
        // }
    }
}

impl Into<JsValue> for BookmarkId {
    fn into(self) -> JsValue {
        self.0.into()
    }
}

impl TryFrom<JsValue> for BookmarkId {
    type Error = DomainError;
    fn try_from(js_value: JsValue) -> Result<Self, Self::Error> {
        serde_wasm_bindgen::from_value(js_value)
            .map_err(|error| DomainError::ParseJsValue(error.to_string()))
    }
}

impl Into<IdBytes> for BookmarkId {
    fn into(self) -> IdBytes {
        let id: u64 = Into::into(self);
        id.to_ne_bytes()
    }
}

impl From<IdBytes> for BookmarkId {
    fn from(id_bytes: IdBytes) -> Self {
        Self(u64::from_ne_bytes(id_bytes))
    }
}

/// JsValueに変換される際に、u64型をバイト配列（Uint8Array型？）に変換させる。
impl Serialize for BookmarkId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let id_bytes: IdBytes = Into::into(*self);
        // serializer.serialize_bytes(&id_bytes) // こちらだとChromeにてエラー発生。invalid type: JsValue(Object({"0":0,"1":0,"2":0,"3":0,"4":0,"5":0,"6":0,"7":0})), expected an array of length 8.
        serializer.serialize_newtype_struct("IdBytes", &id_bytes)
    }
}

/// JsValueから変換される場合に、バイト配列（Uint8Array型？）をu64型に変換させる。
impl<'de> Deserialize<'de> for BookmarkId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id_bytes: IdBytes = Deserialize::deserialize(deserializer)?;
        Ok(Self::from(id_bytes))
    }
}

#[test]
fn endian_test() {
    // let bytes = 0x1234567890123456u64.to_ne_bytes();
    // assert_eq!(
    //     bytes,
    //     if cfg!(target_endian = "big") {
    //         println!("big");
    //         [0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]
    //     } else {
    //         println!("little");
    //         [0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]
    //     }
    // );
    let id: u64 = 643793749666697469;
    let id_bytes = id.to_ne_bytes();
    println!("id_bytes: {id_bytes:?}");
    let back_id = u64::from_ne_bytes(id_bytes);
    println!("back_id: {back_id:?}");
    assert_eq!(id, back_id);
}

// /// JsValueに変換される際に&str型に型変換させる。
// impl Serialize for BookmarkId {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let id_string: String = Into::into(*self);
//         serializer.serialize_str(&id_string)
//     }
// }

// /// JsValueから変換される場合に、文字列をu64に変換させる。
// impl<'de> Deserialize<'de> for BookmarkId {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let string_value: String = Deserialize::deserialize(deserializer)?;
//         match u64::from_str_radix(&string_value, 10) {
//             Ok(res) => Ok(Self(res)),
//             Err(error) => Err(serde::de::Error::custom(format!(
//                 "Failed to deserialize bookmark id: {}",
//                 error
//             ))),
//         }
//     }
// }

#[test]
fn should_deserialize() {
    let string_value = "222n";
    let result = u64::from_str_radix(string_value, 10);
    println!("result: {result:#?}");
}

#[test]
#[cfg(target_arch = "wasm32")]
fn should_serde_bookmark_id() {
    let id = BookmarkId(623677211081183232);

    let serializer =
        serde_wasm_bindgen::Serializer::new().serialize_large_number_types_as_bigints(true);
    let value: JsValue = id.serialize(&serializer).unwrap();
    println!("value: {value:#?}");

    let deserialized_id = serde_wasm_bindgen::from_value::<BookmarkId>(value).unwrap();
    println!("deserialized_id: {deserialized_id:?}");

    assert_eq!(id, deserialized_id);
}

const LINK_TYPE_CODE: u32 = 1;
const FOLDER_TYPE_CODE: u32 = 2;
const TAG_TYPE_CODE: u32 = 3;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TypeCode {
    #[default]
    Link = LINK_TYPE_CODE,
    Folder = FOLDER_TYPE_CODE,
    Tag = TAG_TYPE_CODE,
}

impl TypeCode {
    pub fn is_link(&self) -> bool {
        *self == Self::Link
    }

    pub fn is_folder(&self) -> bool {
        *self == Self::Folder
    }

    pub fn is_tag(&self) -> bool {
        *self == Self::Tag
    }

    pub fn list_type_iter() -> impl Iterator<Item = TypeCode> {
        [Self::Folder, Self::Tag].into_iter()
    }
}

const TYPE_CODE_STRING_LINK: &'static str = "link";
const TYPE_CODE_STRING_FOLDER: &'static str = "folder";
const TYPE_CODE_STRING_TAG: &'static str = "tag";

impl Into<&str> for TypeCode {
    fn into(self) -> &'static str {
        match self {
            Self::Link => TYPE_CODE_STRING_LINK,
            Self::Folder => TYPE_CODE_STRING_FOLDER,
            Self::Tag => TYPE_CODE_STRING_TAG,
        }
    }
}

impl From<u8> for TypeCode {
    fn from(code: u8) -> Self {
        From::from(code as u32)
    }
}

impl From<u32> for TypeCode {
    fn from(code: u32) -> Self {
        match code {
            LINK_TYPE_CODE => Self::Link,
            FOLDER_TYPE_CODE => Self::Folder,
            TAG_TYPE_CODE => Self::Tag,
            _ => Self::Link,
        }
    }
}

impl Into<u32> for TypeCode {
    fn into(self) -> u32 {
        match self {
            Self::Link => LINK_TYPE_CODE,
            Self::Folder => FOLDER_TYPE_CODE,
            Self::Tag => TAG_TYPE_CODE,
        }
    }
}

/// JsValueに変換される際にu32型に型変換させる。
impl Serialize for TypeCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(Into::into(*self))
    }
}

/// JsValueから変換される場合に、数値型(consoleではfloating pointとの表示)をTypeCode型に変換させる。
impl<'de> Deserialize<'de> for TypeCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let number_value: u8 = Deserialize::deserialize(deserializer)?;
        Ok(TypeCode::from(number_value))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct List {
    pub(crate) user_id: UserId,
    pub bookmark_id: BookmarkId,
    pub(crate) parent_id: ParentId,
    pub(crate) type_code: TypeCode,
    pub child_count: ChildCount,
    pub(crate) position: Position,
    pub(crate) created_at: ListTime,
    pub(crate) updated_at: ListTime,
    pub(crate) last_visit: ListTime,
    pub title: Title,
}

impl List {
    pub fn is_same_type_code(&self, type_code: TypeCode) -> bool {
        self.type_code == type_code
    }

    pub fn is_folder(&self) -> bool {
        self.type_code.is_folder()
    }

    pub fn is_tag(&self) -> bool {
        self.type_code.is_tag()
    }

    pub fn builder() -> ListBuilder {
        ListBuilder::new()
    }

    pub fn title(&self) -> &Title {
        &self.title
    }

    pub fn id(&self) -> &BookmarkId {
        &self.bookmark_id
    }
}

#[derive(Debug, Default)]
pub struct ListBuilder {
    user_id: u64,
    bookmark_id: u64,
    parent_id: u64,
    type_code: u32,
    child_count: u32,
    position: u64,
    created_at: i64,
    updated_at: i64,
    last_visit: i64,
    title: String,
}

impl ListBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> List {
        let ListBuilder {
            user_id,
            bookmark_id,
            parent_id,
            type_code,
            child_count,
            position,
            created_at,
            updated_at,
            last_visit,
            title,
        } = self;
        List {
            user_id: user_id.into(),
            bookmark_id: bookmark_id.into(),
            parent_id: parent_id.into(),
            type_code: type_code.into(),
            child_count,
            position,
            created_at,
            updated_at,
            last_visit,
            title: title.into(),
        }
    }

    pub fn set_user_id(mut self, user_id: u64) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn set_bookmark_id(mut self, bookmark_id: u64) -> Self {
        self.bookmark_id = bookmark_id;
        self
    }

    pub fn set_parent_id(mut self, parent_id: u64) -> Self {
        self.parent_id = parent_id;
        self
    }

    pub fn set_type_code(mut self, type_code: u32) -> Self {
        self.type_code = type_code;
        self
    }

    pub fn set_child_count(mut self, child_count: u32) -> Self {
        self.child_count = child_count;
        self
    }

    pub fn set_position(mut self, position: u64) -> Self {
        self.position = position;
        self
    }

    pub fn set_created_at(mut self, created_at: i64) -> Self {
        self.created_at = created_at;
        self
    }

    pub fn set_updated_at(mut self, updated_at: i64) -> Self {
        self.updated_at = updated_at;
        self
    }

    pub fn set_now_updated(mut self) -> Self {
        // let start = SystemTime::now();
        // let since_the_epoch = start
        //     .duration_since(UNIX_EPOCH)
        //     .expect("time went backwards");
        // self.updated_at = since_the_epoch.as_millis() as i64;

        let float_updated_at = instant::now();
        self.updated_at = ((float_updated_at / 100_00.0) as i64) + 100_00;
        self
    }

    pub fn set_last_visit(mut self, last_visit: i64) -> Self {
        self.last_visit = last_visit;
        self
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    // pub fn set_created_at(&mut self, created_at: i64) {
    //     self.created_at = created_at;
    // }

    // pub fn created_at_mut(&mut self) -> &mut i64 {
    //     &mut self.created_at
    // }
}

impl TryInto<JsValue> for &List {
    type Error = serde_wasm_bindgen::Error;
    fn try_into(self) -> Result<JsValue, Self::Error> {
        let serializer = Serializer::new().serialize_large_number_types_as_bigints(true);
        self.serialize(&serializer)
    }
}
