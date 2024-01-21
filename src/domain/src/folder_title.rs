use crate::Title;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FolderTitle {
    New(Title),
    Old(Title),
}

impl FolderTitle {
    pub fn is_empty(&self) -> bool {
        // let d = self.deref();
        // let d = **self;
        // (**self).as_ref().is_empty()
        match self {
            Self::New(title) => title.is_empty(),
            Self::Old(title) => title.is_empty(),
        }
        // false
    }
}
