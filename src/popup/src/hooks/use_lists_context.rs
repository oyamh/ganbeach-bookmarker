use domain::{BookmarkId, Lists, Title, TitleMap, TypeCode};
use std::fmt::{self, Debug};
use std::ops::Deref;
use std::rc::Rc;
use yew::prelude::*;
// use std::cell::{Ref, RefCell};

#[derive(Default)]
pub struct ListsState {
    lists: Lists,
    title_map: TitleMap,
}

impl From<Lists> for ListsState {
    fn from(lists: Lists) -> Self {
        let title_map = TitleMap::from(&lists);
        Self { lists, title_map }
    }
}

impl PartialEq for ListsState {
    fn eq(&self, other: &Self) -> bool {
        self.lists == other.lists
    }
}

impl ListsState {
    pub fn inner<'a>(&'a self) -> &'a Lists {
        &self.lists
    }

    pub fn list_title_by_id(&self, id: &BookmarkId) -> Option<Title> {
        Some(self.lists.list_by_id(id)?.title().to_owned())
    }

    pub fn latest_folder_title(&self) -> Option<Title> {
        self.lists
            .latest_folder()
            .map(|folder| folder.title().to_owned())
    }

    // fn list_id_by_name(&self, type_code: &TypeCode, title: &Title) -> Option<BookmarkId> {
    //     self.title_map
    //         .get(type_code)?
    //         .get(title)
    //         .map(|id| Some(*id))?
    // }
    fn list_id_by_name<T>(&self, type_code: &TypeCode, title: T) -> Option<BookmarkId>
    where
        T: AsRef<str>,
    {
        self.title_map
            .get(type_code)?
            .get(&title.as_ref().into())
            .map(|id| Some(*id))?
    }

    // pub fn folder_id_by_name(&self, title: &Title) -> Option<BookmarkId> {
    //     self.list_id_by_name(&TypeCode::Folder, title)
    // }
    pub fn folder_id_by_name<T>(&self, title: &T) -> Option<BookmarkId>
    where
        T: AsRef<str>,
    {
        self.list_id_by_name(&TypeCode::Folder, title)
    }

    // pub fn tag_id_by_name(&self, title: &Title) -> Option<BookmarkId> {
    //     self.list_id_by_name(&TypeCode::Tag, title)
    // }
    pub fn tag_id_by_name<T>(&self, title: &T) -> Option<BookmarkId>
    where
        T: AsRef<str>,
    {
        self.list_id_by_name(&TypeCode::Tag, title)
    }
}

impl Deref for ListsState {
    type Target = Lists;
    fn deref(&self) -> &Self::Target {
        &self.inner()
    }
}

impl Reducible for ListsState {
    type Action = ListsAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        //WARN: 新しい構造体を返さないと更新イベントが発火されないので注意。
        match action {
            ListsAction::Set(lists) => Rc::new(ListsState::from(lists)),
        }
    }
}

impl Debug for ListsState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

// #[derive(Default)]
// pub struct ListsState {
//     lists: RefCell<Lists>,
//     title_map: RefCell<TitleMap>,
// }

// impl From<Lists> for ListsState {
//     fn from(lists: Lists) -> Self {
//         let title_map = TitleMap::from(&lists);
//         Self {
//             lists: RefCell::new(lists),
//             title_map: RefCell::new(title_map),
//         }
//     }
// }

// impl PartialEq for ListsState {
//     fn eq(&self, other: &Self) -> bool {
//         self.lists == other.lists
//     }
// }

// impl ListsState {
//     pub fn inner<'a>(&'a self) -> Ref<'a, Lists> {
//         self.lists.borrow()
//     }

//     // fn set_lists(&self, new_lists: Lists) {
//     //     if let Ok(mut title_map) = self.title_map.try_borrow_mut() {
//     //         *title_map = TitleMap::from(&new_lists);
//     //     }
//     //     if let Ok(mut lists) = self.lists.try_borrow_mut() {
//     //         *lists = new_lists;
//     //     }
//     // }

//     pub fn list_title_by_id(&self, id: &BookmarkId) -> Option<Title> {
//         Some(self.lists.borrow().list_by_id(id)?.title().to_owned())
//     }

//     pub fn latest_folder_title(&self) -> Option<Title> {
//         self.lists
//             .borrow()
//             .latest_folder()
//             .map(|folder| folder.title().to_owned())
//     }

//     fn list_id_by_name(&self, type_code: &TypeCode, title: &Title) -> Option<BookmarkId> {
//         self.title_map
//             .borrow()
//             .get(type_code)?
//             .get(title)
//             .map(|id| Some(*id))?
//     }

//     pub fn folder_id_by_name(&self, title: &Title) -> Option<BookmarkId> {
//         self.list_id_by_name(&TypeCode::Folder, title)
//     }

//     pub fn tag_id_by_name(&self, title: &Title) -> Option<BookmarkId> {
//         self.list_id_by_name(&TypeCode::Tag, title)
//     }
// }

// impl Deref for ListsState {
//     type Target = RefCell<Lists>;
//     fn deref(&self) -> &Self::Target {
//         &self.lists
//     }
// }

pub enum ListsAction {
    Set(Lists),
}

// impl Reducible for ListsState {
//     type Action = ListsAction;
//     fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
//         // //ここで同じ構造体を使いまわしていたところ、更新が走らなくなった。
//         // match action {
//         //     ListsAction::Set(lists) => {
//         //         self.set_lists(lists);
//         //         //ここで同じ構造体を使いまわしていたところ、更新が走らなくなった。
//         //         // Rc::new(Self::from(lists))
//         //     }
//         // };
//         // self.into()
//         match action {
//             ListsAction::Set(lists) => {
//                 //NOTE: 新しい構造体を返さないと更新イベントが発火されない。
//                 log::debug!("ListsStatereduce ListAction::Set lists.len={}", lists.len());
//                 Rc::new(Self::from(lists))
//             }
//         }
//     }
// }

// impl Debug for ListsState {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.inner())
//     }
// }

// pub type ListsContext = UseReducerHandle<ListsState>;
pub type ListsContext = UseReducerHandle<ListsState>;

#[hook]
pub fn use_lists_context() -> ListsContext {
    use_context::<ListsContext>().expect("no lists context found")
}

// #[derive(Clone)]
// pub struct UseListsContextHandle(ListsContext);

// impl Deref for UseListsContextHandle {
//     type Target = ListsState;
//     fn deref(&self) -> &Self::Target {
//         &(*self.0)
//     }
// }

// impl PartialEq for UseListsContextHandle {
//     fn eq(&self, other: &Self) -> bool {
//         *self.0 == *other.0
//     }
// }

// impl fmt::Debug for UseListsContextHandle {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // write!(f, "UseListsContextHandle")
//         f.debug_struct("UseListsContextHandle")
//             .field("value", &format!("{:?}", *self.0))
//             .finish()
//     }
// }

// #[hook]
// pub fn use_lists_context() -> UseListsContextHandle {
//     let inner = use_context::<ListsContext>().expect("no lists context found");
//     UseListsContextHandle(inner)
// }
