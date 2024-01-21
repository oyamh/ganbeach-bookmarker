use crate::client::Client;
use crate::gooscut::bookmark_getter_client::BookmarkGetterClient as PbBookmarkGetterClient;
use crate::ServerError;
use domain::{
    BookmarkReader, DomainError, GetAllListsCommand, Lists, Metadata, MetadataHeaders, ServerUrl,
};

fn generate_bookmark_getter_client<T>(metadata: T) -> PbBookmarkGetterClient<Client<T>>
where
    T: MetadataHeaders,
{
    PbBookmarkGetterClient::new(Client::new(ServerUrl::BookmarkGetter, metadata))
}

#[derive(Default)]
pub struct BookmarkGetterClient;

impl BookmarkGetterClient {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BookmarkReader for BookmarkGetterClient {
    async fn fetch_all_lists(
        &self,
        command: GetAllListsCommand,
        metadata: Metadata,
    ) -> Result<Lists, DomainError> {
        let mut client = generate_bookmark_getter_client(metadata);
        let response = client
            .get_all_lists(command)
            .await
            .map_err(Into::<ServerError>::into)?;
        Ok(response.into_inner().into())
    }
}

#[tokio::test]
async fn should_get_all_lists() {
    use domain::AccessToken;
    let access_token = "";
    let inner = GetAllListsCommand {};
    // let request = Command::builder()
    //     .with_inner(inner)
    //     .with_access_token(access_token)
    //     .build();
    let metadata = Metadata::new().with_access_token(AccessToken::from(access_token));
    let response = BookmarkGetterClient {}
        .fetch_all_lists(inner, metadata)
        .await;
    println!("{:?}", response);
}

// struct BookmarkGetter<T>
// where
//     T: RequestGetter,
// {
//     metadata: PhantomData<T>,
// }

// impl<T> BookmarkGetter<T>
// where
//     T: RequestGetter,
// {
//     pub fn new(metadata: T) -> BookmarkGetterClient<Client<T>> {
//         BookmarkGetterClient::new(Client::new(ServerUrl::BookmarkGetter, metadata))
//     }
// }

// pub struct GetAllListsClient;

// #[async_trait(?Send)]
// impl Clienter for GetAllListsClient {
//     type Request = Request<GetAllListsRequest>;
//     type Response = Result<Lists, ClientError>;
//     async fn run(request: Self::Request) -> Self::Response {
//         let mut client = BookmarkGetter::new(request.metadata);
//         let inner: PbGetAllListsRequest = request.inner.unwrap().into();
//         let response = client.get_all_lists(inner).await?;
//         log::debug!("response: {response:?}");
//         Ok(response.into_inner().into())
//     }
// }

// pub async fn get_all_lists_old(access_token: AccessToken) -> Result<Lists, ClientError> {
//     let mut client = BookmarkGetterClient::new(Client::new(BOOKMARK_BASE_URL, ""));

//     let mut request = Request::new(GetAllListsRequest {});
//     request.set_timeout(Duration::from_secs(120));
//     request
//         .metadata_mut()
//         .insert("authorization", access_token.header().parse().unwrap());

//     let response = client.get_all_lists(request).await?;

//     log::debug!("response: {response:?}");

//     Ok(response.into_inner().into())
// }

// #[tokio::test]
// async fn should_get_all_lists() {
//     let access_token = "";
//     let result = get_all_lists_old(AccessToken::new(access_token)).await;
//     println!("{:?}", result);
// }
