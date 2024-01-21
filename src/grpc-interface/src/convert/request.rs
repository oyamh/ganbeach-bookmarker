use crate::gooscut::{
    AccessTokenRequest as PbAccessTokenRequest, AddBookmarksRequest as PbAddBookmarksRequest,
    GetAllListsRequest as PbGetAllListsRequest, LoginUrlRequest as PbLoginUrlRequest,
    NewBookmarkData as PbNewBookmarkData,
};
use domain::{
    AccessTokenCommand, CreateBookmarkCommand, FolderTitle, GetAllListsCommand, LoginUrlCommand,
    NewBookmarkData,
};
use tonic::{IntoRequest, Request};

impl IntoRequest<PbGetAllListsRequest> for GetAllListsCommand {
    fn into_request(self) -> Request<PbGetAllListsRequest> {
        Request::new(PbGetAllListsRequest::from(self))
    }
}

impl From<GetAllListsCommand> for PbGetAllListsRequest {
    fn from(_: GetAllListsCommand) -> Self {
        Self {}
    }
}

impl From<NewBookmarkData> for PbNewBookmarkData {
    fn from(src: NewBookmarkData) -> Self {
        Self {
            type_code: src.type_code.into(),
            title: src.name.into(),
            url: src.url.into(),
            annotation: src.annotation.into(),
        }
    }
}

// impl From<CreateBookmarkCommand> for PbAddBookmarkRequest {
//     fn from(command: CreateBookmarkCommand) -> Self {
//         let first = command.first().expect("expect at least 1 length");
//         Self {
//             title: first.name.to_owned().into(),
//             url: first.url.to_owned().into(),
//             type_code: first.type_code.into(),
//             parent_id: command.folder_id.into(),
//             tag_ids: command.tag_ids.into(),
//             unknown_tags: command.unknown_tag_names.into(),
//         }
//     }
// }

impl IntoRequest<PbAddBookmarksRequest> for CreateBookmarkCommand {
    fn into_request(self) -> Request<PbAddBookmarksRequest> {
        Request::new(PbAddBookmarksRequest::from(self))
    }
}

impl From<CreateBookmarkCommand> for PbAddBookmarksRequest {
    fn from(command: CreateBookmarkCommand) -> Self {
        let new_folders = match command.folder_title {
            FolderTitle::New(new_folder_title) => vec![new_folder_title.into()],
            FolderTitle::Old(_old_folder_title) => vec![],
        };
        let new_bookmarks = command
            .new_bookmarks
            .into_iter()
            .map(Into::<PbNewBookmarkData>::into)
            .collect();
        Self {
            parent_id: command.folder_id.unwrap_or_default().into(),
            new_bookmarks,
            tag_ids: command.tag_ids.into(),
            new_tags: command.new_tags.into(),
            new_folders,
        }
    }
}

impl IntoRequest<PbAccessTokenRequest> for AccessTokenCommand {
    fn into_request(self) -> Request<PbAccessTokenRequest> {
        Request::new(PbAccessTokenRequest::from(self))
    }
}

impl From<AccessTokenCommand> for PbAccessTokenRequest {
    fn from(req: AccessTokenCommand) -> Self {
        Self {
            old_access_token: req
                .old_access_token
                .try_into()
                .expect("invalid old_access_token"),
        }
    }
}

impl From<LoginUrlCommand> for PbLoginUrlRequest {
    fn from(req: LoginUrlCommand) -> Self {
        Self {
            provider: req.provider.try_into().expect("invalid provider"),
        }
    }
}

// impl<T1, T2> From<Request<T1>> for RequestWrapper<T2>
// where
//     T1: 'static,
//     T2: From<T1> + 'static,
// {
//     fn from(req: Request<T1>) -> Self {
//         let inner = req.inner.expect("extract inner Request");
//         let url = req.url;
//         RequestWrapper::new(T2::from(inner), url)
//         // .with_timeout(req.timeout())
//         // .with_access_token(req.access_token())
//         // .with_refresh_token(req.refresh_token())
//     }
// }

// #[derive(Debug)]
// pub struct RequestWrapper<T> {
//     // inner: TonicRequest<T>,
//     inner: Option<T>,
//     url: ServerUrl,
//     metadata: Metadata,
// }

// impl<T> RequestWrapper<T> {
//     fn new(inner: T, url: ServerUrl) -> Self {
//         Self {
//             // inner: TonicRequest::new(inner),
//             inner: Some(inner),
//             url,
//             metadata: Metadata::new(),
//         }
//     }

//     // fn get_ref(&self) -> &T {
//     //     self.inner.get_ref()
//     // }
//     // fn get_mut(&mut self) -> &mut T {
//     //     self.inner.get_mut()
//     // }

//     /// Consumes self
//     pub(crate) fn inner(&mut self) -> TonicRequest<T> {
//         TonicRequest::new(self.inner.take().unwrap())
//     }
// }
