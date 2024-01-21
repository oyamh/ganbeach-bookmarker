use domain::{
    AccessTokenCommand, AccessTokenValue, Accounter, Command, DomainError, LoginUrl,
    LoginUrlCommand, Metadata, MetadataHeaders, ServerUrl,
};

use crate::client::Client;
use crate::gooscut::{
    accounter_client::AccounterClient as PbAccounterClient, LoginUrlRequest as PbLoginUrlRequest,
};
use crate::ServerError;

fn generate_accounter_client<T>(metadata: T) -> PbAccounterClient<Client<T>>
where
    T: MetadataHeaders,
{
    PbAccounterClient::new(Client::new(ServerUrl::Accounter, metadata))
}

#[derive(Default)]
pub struct AccounterClient;

impl AccounterClient {
    pub fn new() -> Self {
        Self::default()
    }

    // pub async fn send(
    //     &self,
    //     // command: impl tonic::IntoRequest<REQ>,
    //     request: AccounterRequest,
    //     metadata: Metadata,
    // ) -> Result<AccounterResponse, DomainError> {
    //     let mut client = generate_accounter_client(metadata);
    //     Ok(match request {
    //         AccounterRequest::AccessToken(request) => AccounterResponse::AccessTokenValue(
    //             client
    //                 .generate_access_token(request)
    //                 .await
    //                 .map_err(Into::<ServerError>::into)?
    //                 .into_inner()
    //                 .into(),
    //         ),
    //         AccounterRequest::LoginUrl(request) => AccounterResponse::LoginUrl(
    //             client
    //                 .generate_login_url(request)
    //                 .await?
    //                 .into_inner()
    //                 .into(),
    //         ),
    //     })
    //     // let response = client
    //     //     .generate_access_token(command.unwrap())
    //     //     .await
    //     //     .map_err(Into::<ServerError>::into)?;
    //     // log::debug!("response: {response:?}");
    //     // Ok(response.into_inner())
    // }
}

impl Accounter for AccounterClient {
    async fn generate_access_token(
        &self,
        command: AccessTokenCommand,
        metadata: Metadata,
    ) -> Result<AccessTokenValue, DomainError> {
        log::debug!("generate_access_token");
        let mut client = generate_accounter_client(metadata);
        let response = client
            .generate_access_token(command)
            .await
            .map_err(Into::<ServerError>::into)?;
        log::debug!("response: {response:?}");
        Ok(response.into_inner().try_into()?)
    }

    async fn generate_login_url(
        &self,
        request: Command<LoginUrlCommand>,
    ) -> Result<LoginUrl, DomainError> {
        let mut client = generate_accounter_client(request.metadata);
        let inner: PbLoginUrlRequest = request.inner.unwrap().into();
        let response = client
            .generate_login_url(inner)
            .await
            .map_err(Into::<ServerError>::into)?;
        Ok(response.into_inner().try_into()?)
    }
}

#[tokio::test]
async fn should_generate_access_token() {
    const TESTING_REFRESH_TOKEN: &'static str = "refresh-token=; nonce=";
    let inner = AccessTokenCommand {
        old_access_token: "".to_string(),
    };
    let request = Command::builder()
        .with_inner(inner)
        .with_refresh_token(&TESTING_REFRESH_TOKEN)
        .build();
    let response = AccounterClient {}
        .generate_access_token(request.inner.unwrap(), request.metadata)
        .await;
    println!("response: {:?}", response);
}

#[tokio::test]
async fn should_generate_login_url() {
    let inner = LoginUrlCommand {
        provider: "github".to_string(),
    };
    let request = Command::builder().with_inner(inner).build();
    let response = AccounterClient {}.generate_login_url(request).await;
    println!("response: {:?}", response);
}

// pub struct GenerateAccessTokenClient;

// #[async_trait(?Send)]
// impl Clienter for GenerateAccessTokenClient {
//     type Request = Request<AccessTokenRequest>;
//     type Response = Result<AccessTokenValue, ClientError>;
//     async fn run(request: Self::Request) -> Self::Response {
//         let mut client = generate_accounter_client(request.metadata);
//         let inner: PbAccessTokenRequest = request.inner.unwrap().into();
//         let response = client.generate_access_token(inner).await?;
//         log::debug!("response: {response:?}");
//         Ok(response.into_inner().into())
//     }
// }

// #[tokio::test]
// async fn should_generate_access_token() {
//     const TESTING_REFRESH_TOKEN: &'static str = "refresh-token=; nonce=";
//     let inner = AccessTokenRequest {
//         old_access_token: "".to_string(),
//     };
//     let request = Request::builder()
//         .with_inner(inner)
//         .with_refresh_token(&TESTING_REFRESH_TOKEN)
//         .build();
//     let response = GenerateAccessTokenClient::run(request).await;
//     println!("{:?}", response);
// }

// pub struct GenerateLoginUrlClient;

// #[async_trait(?Send)]
// impl Clienter for GenerateLoginUrlClient {
//     type Request = Request<LoginUrlRequest>;
//     type Response = Result<LoginUrl, ClientError>;
//     async fn run(request: Self::Request) -> Self::Response {
//         let mut client = generate_accounter_client(request.metadata);
//         let inner: PbLoginUrlRequest = request.inner.unwrap().into();
//         let response = client.generate_login_url(inner).await?;
//         Ok(response.into_inner().into())
//     }
// }

// #[tokio::test]
// async fn should_generate_login_url() {
//     let inner = LoginUrlRequest {
//         provider: "github".to_string(),
//     };
//     let request = Request::builder().with_inner(inner).build();
//     let response = GenerateLoginUrlClient::run(request).await;
//     println!("{:?}", response);
// }

// pub struct Accounter<T>
// where
//     T: RequestGetter,
// {
//     metadata: PhantomData<T>,
// }

// // 引数からmetadataを取り出す手段が分からなかったので不採用。
// pub struct AllClient<RQ, RS> {
//     request: PhantomData<RQ>,
//     response: PhantomData<RS>,
// }
// #[async_trait(?Send)]
// impl<RQ, RS> Clienter for AllClient<RQ, RS> {
//     type Request = Request<RQ>;
//     type Response = Result<RS, ClientError>;
//     async fn run(&self, request: Self::Request) -> Self::Response {
//         let mut client = PbAccounterClient::new(Client::new(ServerUrl::Accounter, request.metadata)); // NOTE: ここ。
//         Err(ClientError::Unreachable)
//     }
// }

// pub async fn generate_access_token<'a>(
//     old_access_token: AccessToken,
//     refresh_token: &'a str,
// ) -> Result<AccessTokenValue, ClientError> {
//     let mut client = PbAccounterClient::new(
//         Client::new(BASE_URL).with_refresh_token(Some(refresh_token.to_string())),
//     );

//     let request = PbAccessTokenRequest {
//         old_access_token: old_access_token.to_string(),
//     };

//     let response = client.generate_access_token(request).await?;
//     println!("{:?}", response);

//     match response.into_inner() {
//         message if message.access_token.len() != 0 => Ok(AccessTokenValue::Token(
//             AccessToken::new(&*message.access_token),
//         )),
//         message if message.login_url.len() != 0 => {
//             Ok(AccessTokenValue::LoginUrl(LoginUrl::new(message.login_url)))
//         }
//         _ => Err(ClientError::Unreachable),
//     }
// }

// #[tokio::test]
// async fn should_send_grpc_request() {
//     const TESTING_REFRESH_TOKEN: &'static str = "refresh-token=; nonce=";
//     let response = generate_access_token(AccessToken::default(), TESTING_REFRESH_TOKEN).await;
//     println!("{:?}", response);
// }
// pub async fn generate_login_url(
//     provider: &str,
// ) -> Result<tonic::Response<PbLoginUrlResponse>, tonic::Status> {
//     let mut client = PbAccounterClient::new(Client::new(BASE_URL));

//     let request = PbLoginUrlRequest {
//         provider: provider.to_string(),
//     };

//     client.generate_login_url(request).await
// }

// #[tokio::test]
// async fn should_generate_login_url() {
//     let response = generate_login_url("github").await;
//     println!("{:?}", response);
// }
