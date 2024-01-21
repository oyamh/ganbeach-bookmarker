use crate::client::Client;
use crate::gooscut::bookmark_adder_client::BookmarkAdderClient as PbBookmarkCreatorClient;
use crate::ServerError;
use domain::{
    BookmarkCreator, Command, CreateBookmarkCommand, CreateBookmarkResult, DomainError, Metadata,
    MetadataHeaders, ServerUrl,
};

#[derive(Default)]
pub struct BookmarkCreatorClient;

impl BookmarkCreatorClient {
    pub fn new() -> Self {
        Default::default()
    }
}

fn generate_bookmark_creator_client<T>(metadata: T) -> PbBookmarkCreatorClient<Client<T>>
where
    T: MetadataHeaders,
{
    PbBookmarkCreatorClient::new(Client::new(ServerUrl::BookmarkCreator, metadata))
}

impl BookmarkCreator for BookmarkCreatorClient {
    async fn create_bookmark(
        &self,
        _command: Command<CreateBookmarkCommand>,
    ) -> Result<CreateBookmarkResult, DomainError> {
        unimplemented!()
        // let mut client = generate_bookmark_creator_client(command.metadata);
        // let request: PbAddBookmarkRequest = command.inner.unwrap().into();
        // let response = client
        //     .add_bookmark(request)
        //     .await
        //     .map_err(Into::<ClientError>::into)?;
        // Ok(response.into_inner().into())
    }

    async fn create_bookmarks(
        &self,
        command: CreateBookmarkCommand,
        metadata: Metadata,
    ) -> Result<CreateBookmarkResult, DomainError> {
        let mut client = generate_bookmark_creator_client(metadata);
        println!("{command:#?}");
        let response = client
            .add_bookmarks(command)
            .await
            .map_err(Into::<ServerError>::into)?;
        Ok(response.into_inner().into())
    }
}

mod tests {
    #[allow(unused_imports)]
    use domain::AccessToken;

    #[tokio::test]
    async fn should_create_multiple_bookmarks_with_folder_and_tags() {
        use super::*;
        use domain::Titles;
        //TODO: 複数ブックマークができるか試す。
        let access_token = AccessToken::from("");
        let tag_ids: Vec<domain::BookmarkId> =
            vec![695984634218942693.into(), 695984634210553860.into()]; //rust, todo
        let new_tag_titles: Titles = vec!["newtagtest1", "newtagtest2"].into();
        let mut command = CreateBookmarkCommand::new(
            "first test link".into(),
            "http://pppposttestttt.com/url/1".into(),
            domain::TypeCode::Link,
            Some(695984624865647265.into()), //mobile
            tag_ids.into(),
            new_tag_titles,
            domain::FolderTitle::New("new folder add test".into()),
            "annotation".into(),
        );
        command.push(
            "second test link".into(),
            "http://pppposttestttt.com/url/2".into(),
            domain::TypeCode::Link,
            "annotation".into(),
        );
        command.push(
            "third test link".into(),
            "http://pppposttestttt.com/url/3".into(),
            domain::TypeCode::Link,
            "annotation".into(),
        );
        // let command = Command::builder()
        //     .with_inner(inner)
        //     .with_access_token(access_token)
        //     .build();
        let metadata = Metadata::new().with_access_token(access_token);
        println!("command={command:#?}");
        let response = BookmarkCreatorClient {}
            .create_bookmarks(command, metadata)
            .await;
        println!("response={:?}", response);
        assert!(response.is_ok())
    }

    #[tokio::test]
    async fn should_create_single_bookmark_with_folder() {
        use super::*;
        use domain::Titles;
        let access_token = AccessToken::from("");
        let tag_ids: Vec<domain::BookmarkId> = Vec::new();
        let command = CreateBookmarkCommand::new(
            "post test link".into(),
            "http://pppposttestttt.com/url/2".into(),
            domain::TypeCode::Link,
            Some(695984624865647265.into()), //mobile
            tag_ids.into(),
            Titles::default(),
            domain::FolderTitle::New("new folder add test".into()),
            "".into(),
        );
        // let command = Command::builder()
        //     .with_inner(inner)
        //     .with_access_token(access_token)
        //     .build();
        println!("{command:#?}");
        let metadata = Metadata::new().with_access_token(access_token);
        let response = BookmarkCreatorClient {}
            .create_bookmarks(command, metadata)
            .await;
        println!("response: {:?}", response);
        assert!(response.is_ok())
    }
}

// //TODO: type_codeがfolder, tagなら、backgroundのデータに追加する。
// //TODO: 追加したbookmark_idを適切なfolder, tagのchild_idsに追加する。
// //上記の二つはhandlerで処理する。
// #[tokio::test]
// async fn should_create_new_list() {
//     let access_token = "";
//     let tag_ids: Vec<domain::BookmarkId> = Vec::new();
//     let inner = CreateBookmarkCommand::new(
//         "post test 2"..into(),
//         "post test url 2"..into(),
//         domain::TypeCode::Folder,
//         Some(643793741970149376.into()),
//         tag_ids.into(),
//         Vec::new().into(),
//         "post test folder title 2"..into(),
//     );
//     let command = Command::builder()
//         .with_inner(inner)
//         .with_access_token(access_token)
//         .build();
//     let response = BookmarkCreatorClient {}.create_bookmark(command).await;
//     println!("response: {:?}", response);
// }
