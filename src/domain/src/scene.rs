use crate::{BookmarkId, TagIds, Title};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Scene {
    #[default]
    Input,
    Created {
        folder_id: BookmarkId,
        folder_title: Title,
        tag_ids: TagIds,
        //TODO: 追加したブックマークのbookmark_idも設定させる。URLのクエリーかハッシュタグにid=idとして追加して、飛んだ先で該当行にfocusさせる。
        //new_id: BookmarkId,
    },
    Creating,
    Error {
        message: String,
    },
}

impl AsRef<str> for Scene {
    fn as_ref(&self) -> &str {
        match self {
            Scene::Input => "input",
            Scene::Created { .. } => "created",
            Scene::Creating => "creating",
            Scene::Error { .. } => "error",
        }
    }
}
