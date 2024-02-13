use std::{
    cell::{Ref, RefCell},
    collections::{hash_map, HashMap},
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::{
    BookmarkId, DatabaseAccessor, DatabaseAccessorProvider, DomainError, List, SuggestInfo, Title,
    TypeCode,
};

type ListMap = HashMap<BookmarkId, List>;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Lists(pub ListMap);

impl Lists {
    pub fn new(lists: Vec<List>) -> Self {
        Self(
            lists
                .into_iter()
                .map(|list| (list.bookmark_id, list))
                .collect::<ListMap>(),
        )
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    // pub fn extend_lists(mut self, lists: Lists) -> Self {
    //     // self.0.insert(list.bookmark_id, list);
    //     self.0.extend(lists.0);
    //     self
    // }

    // pub fn extend(&mut self, lists: Lists) {
    //     // // self.0.insert(list.bookmark_id, list);
    //     // self.0.extend(lists.0);
    //     self.0.extend(lists.0)
    // }

    pub fn list_by_id(&self, id: &BookmarkId) -> Option<&List> {
        self.0.get(id)
    }

    pub fn typed_lists_iter(&self, type_code: TypeCode) -> impl Iterator<Item = &List> {
        self.0
            .values()
            .filter(move |data| data.type_code == type_code)
    }

    // pub fn tags_iter(&self) -> impl Iterator<Item = &List> {
    //     self.0
    //         .iter()
    //         .filter(|(_id, data)| data.is_tag())
    //         .map(|(_id, data)| data)
    // }

    pub fn tag_titles_iter(&self) -> impl Iterator<Item = &Title> {
        self.0
            .values()
            .filter(|list| list.is_tag())
            .map(|list| -> &Title { &list.title })
    }

    pub fn latest_folder(&self) -> Option<&List> {
        // log::debug!("lists.0: {:?}", self.0);
        self.0
            .values()
            .filter(|list| TypeCode::is_folder(&list.type_code))
            .max_by(|a, b| a.updated_at.cmp(&b.updated_at))
    }

    pub fn recent_tags_titles_iter(&self) -> impl Iterator<Item = &List> {
        let mut filtered_list_vec = self
            .0
            .values()
            .filter(|list| TypeCode::is_tag(&list.type_code))
            .collect::<Vec<&List>>();
        filtered_list_vec.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));
        filtered_list_vec.into_iter()
    }

    pub async fn put_all<T>(&self, ctx: &T) -> Result<(), DomainError>
    where
        T: DatabaseAccessorProvider<Error = DomainError>,
    {
        let accessor = DatabaseAccessorProvider::provide(ctx);

        let js_value_iter = self
            .into_iter()
            .map(|list| TryInto::<JsValue>::try_into(list).unwrap());
        accessor.clear().await?;
        accessor.put_iter(js_value_iter).await?;
        Ok(())
    }

    pub async fn get_all<T>(ctx: &T) -> Result<Self, DomainError>
    where
        T: DatabaseAccessorProvider<Error = DomainError>,
    {
        let accessor = DatabaseAccessorProvider::provide(ctx);
        Self::try_from(accessor.get_all().await?)
    }
}

/// 文字列キー以外のHashMapをSerialize及びDeserializeできるようにするための処理。
/// 文字列キー以外のHashMapは、そのままではJsValueに変換できない。
/// なので配列にして変換して、再度HashMapに戻す。
impl Serialize for Lists {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.0.values())
    }
}

impl<'de> Deserialize<'de> for Lists {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::new(Vec::<List>::deserialize(deserializer)?))
    }
}

impl Display for Lists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl From<Vec<List>> for Lists {
    fn from(list_vec: Vec<List>) -> Self {
        Self::new(list_vec)
    }
}

impl SuggestInfo for List {
    fn title(&self) -> &Title {
        &self.title
    }
    fn child_count(&self) -> u32 {
        self.child_count
    }
}

impl<'a> IntoIterator for &'a Lists {
    type Item = &'a List;
    type IntoIter = hash_map::Values<'a, BookmarkId, List>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.values()
    }
}

impl TryFrom<JsValue> for Lists {
    type Error = DomainError;
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(serde_wasm_bindgen::from_value(value)?)
    }
}

#[derive(Debug, Default)]
pub struct TitleMap(HashMap<TypeCode, HashMap<Title, BookmarkId>>);

impl Deref for TitleMap {
    type Target = HashMap<TypeCode, HashMap<Title, BookmarkId>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TitleMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<RefCell<Lists>> for TitleMap {
    fn from(lists: RefCell<Lists>) -> Self {
        Self::from(lists.borrow())
    }
}

impl From<Ref<'_, Lists>> for TitleMap {
    fn from(lists: Ref<Lists>) -> Self {
        // Self::from(lists.deref())
        Self::from(&*lists)
    }
}

impl From<&Lists> for TitleMap {
    fn from(lists: &Lists) -> Self {
        let mut title_map = Self::default();
        TypeCode::list_type_iter().for_each(|type_code| {
            let list_map = lists
                .into_iter()
                .filter(|list| list.type_code == type_code)
                .map(|list| (list.title.clone(), list.bookmark_id))
                .collect::<HashMap<Title, BookmarkId>>();
            title_map.insert(type_code, list_map);
        });
        title_map
    }
}

// impl<'a> FromIterator<&'a List> for TitleMap {
//     fn from_iter<I>(iter: I) -> Self
//     where
//         I: IntoIterator<Item = &'a List>,
//     {
//         let mut title_map = Self::default();

//         iter.into_iter().for_each(|list| {
//             let type_code = list.type_code;
//             title_map
//                 .entry(type_code)
//                 .or_default()
//                 .insert(list.title.clone(), list.bookmark_id);
//         });

//         title_map
//     }
// }

// impl<'a> FromIterator<&'a List> for TitleMap {
//     fn from_iter<I>(iter: I) -> Self
//     where
//         I: IntoIterator<Item = &'a List>,
//     {
//         let mut title_map = Self::default();

//         let list_iter = iter.into_iter();
//         TypeCode::list_type_iter().for_each(|type_code| {
//             let list_map = list_iter
//                 .filter(|list| list.type_code == type_code)
//                 .map(|list| (list.title.clone(), list.bookmark_id))
//                 .collect::<HashMap<Title, BookmarkId>>();
//             title_map.insert(type_code, list_map);
//         });

//         title_map
//     }
// }

#[cfg(test)]
mod lists_test {
    use super::*;

    #[test]
    fn should_max_by() {
        let list_vec = vec![
            List::builder()
                .set_bookmark_id(1)
                .set_type_code(TypeCode::Folder.into())
                .set_updated_at(1)
                .build(),
            List::builder()
                .set_bookmark_id(2)
                .set_type_code(TypeCode::Folder.into())
                .set_updated_at(2)
                .build(),
            List::builder()
                .set_bookmark_id(3)
                .set_type_code(TypeCode::Folder.into())
                .set_updated_at(3)
                .build(),
        ];
        let lists = Lists::new(list_vec);
        let f = lists.latest_folder();
        println!("{f:#?}");
    }

    #[test]
    fn should_lists_from_vec() {
        let list_vec = vec![
            List::builder()
                .set_bookmark_id(1)
                .set_title("list1".to_string())
                .build(),
            List::builder()
                .set_bookmark_id(2)
                .set_title("list2".to_string())
                .build(),
            List::builder()
                .set_bookmark_id(3)
                .set_title("list3".to_string())
                .build(),
        ];
        let lists = Lists::from(list_vec);
        assert_eq!(3, lists.len());
    }

    #[test]
    fn should_overwrite_same_bookmark_id() {
        let list_vec = vec![
            List::builder()
                .set_bookmark_id(1)
                .set_title("list1".to_string())
                .build(),
            List::builder()
                .set_bookmark_id(1)
                .set_title("list2".to_string())
                .build(),
            List::builder()
                .set_bookmark_id(1)
                .set_title("list3-overwrited".to_string())
                .build(),
        ];
        let lists = Lists::from(list_vec);
        assert_eq!(1, lists.len());
    }

    #[test]
    fn should_title_map_from_lists_pointer() {
        let list_1 = List::builder()
            .set_bookmark_id(1)
            .set_title("list1".to_string())
            .set_type_code(TypeCode::Link.into())
            .build();
        let list_2 = List::builder()
            .set_bookmark_id(2)
            .set_title("list2".to_string())
            .set_type_code(TypeCode::Folder.into())
            .build();
        let list_3 = List::builder()
            .set_bookmark_id(3)
            .set_title("list3".to_string())
            .set_type_code(TypeCode::Tag.into())
            .build();
        let list_4 = List::builder()
            .set_bookmark_id(4)
            .set_title("list4".to_string())
            .set_type_code(TypeCode::Tag.into())
            .build();
        let list_vec = vec![list_1, list_2, list_3, list_4];
        let lists = Lists::from(list_vec);
        let title_map = TitleMap::from(&lists);
        println!("{title_map:#?}");
        assert_eq!(2, title_map.len());
        assert_eq!(1, title_map.get(&TypeCode::Folder).unwrap().len());
        assert_eq!(2, title_map.get(&TypeCode::Tag).unwrap().len());
    }
}
