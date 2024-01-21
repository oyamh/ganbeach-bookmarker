use domain::{MetadataHeaders, ServerUrl};
use std::{future::Future, pin::Pin};

use crate::{GrpcResponse, ServerError, GRPC_HTTP_HEADER};
use bytes::{Bytes, BytesMut};
use http::{
    header::{ACCEPT, CONTENT_TYPE},
    Request as HttpRequest, Response,
};
use http_body::{combinators::UnsyncBoxBody, Body};
use reqwest::RequestBuilder;
use tonic::body::BoxBody;
use tower::Service;

#[derive(Debug, Clone)]
pub struct Client<T>
where
    T: MetadataHeaders,
{
    url: ServerUrl,
    metadata: T,
}

impl<T> Client<T>
where
    T: MetadataHeaders,
{
    pub fn new(url: ServerUrl, metadata: T) -> Self {
        log::debug!("Client::new - url:{url}");
        // std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        //TODO: meatadataを定義しないとダメか？
        Self { url, metadata }
    }
}

impl<T> Service<HttpRequest<BoxBody>> for Client<T>
where
    T: MetadataHeaders,
{
    type Response = Response<UnsyncBoxBody<Bytes, ServerError>>;
    type Error = ServerError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    //TODO: callのテストを書く。
    //send_requestのテストでは異常がなかったから。
    fn call(&mut self, request: HttpRequest<BoxBody>) -> Self::Future {
        let builder = build_request(&self.metadata, self.url, &request);
        match builder {
            Ok(builder) => Box::pin(send_request(builder, request)),
            Err(err) => Box::pin(async { Err(err) }),
        }
    }

    // fn call(&mut self, req: HttpRequest<BoxBody>) -> Self::Future {
    //     Box::pin(request(
    //         self.base_url
    //             .take()
    //             .expect("self.base_url is empty. client should be used only once"),
    //         self.access_token.take().unwrap_or_default(),
    //         self.refresh_token.take().unwrap_or_default(),
    //         req,
    //     ))
    // }

    // //&selfがFutureを跨いでいるのでlifetimeが必要。しかしlifetime設定方法がわからない。
    // fn call(&mut self, req: HttpRequest<BoxBody>) -> Self::Future {
    //     Box::pin(self.send_request(req))
    // }

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        // Ok(()).into()
        std::task::Poll::Ready(Ok(()))
    }
}

//TODO: callのテストを書く。
#[test]
fn should_call_service() {
    unimplemented!();
}

fn build_request<T>(
    metadata: &T,
    url: ServerUrl,
    req: &HttpRequest<BoxBody>,
) -> Result<RequestBuilder, ServerError>
where
    T: MetadataHeaders,
{
    log::debug!("build_request: url:{url}");
    // req: Request { method: POST, uri: /gooscut.Accounter/GenerateAccessToken, version: HTTP/2.0, headers: {"te": "trailers", "content-type": "application/grpc"}, body: UnsyncBoxBody }
    log::debug!("req: {:?}", req);
    // url.push_str(&req.uri().to_string());
    let url = format!("{}{}", url, req.uri());

    let client = reqwest::Client::new();
    let mut builder = client.post(&url);

    builder = builder
        .header(CONTENT_TYPE, GRPC_HTTP_HEADER)
        .header(ACCEPT, GRPC_HTTP_HEADER)
        .header("x-grpc-web", "1");

    // metadata
    //     .headers()
    //     .into_iter()
    //     .map(|(key, value)| builder = builder.header(key, value));
    // req.headers()
    //     .iter()
    //     .filter(|(name, value)| *name != CONTENT_TYPE && *name != ACCEPT)
    //     .map(|(name, value)| builder = builder.header(name, value));

    for (key, value) in metadata.headers() {
        builder = builder.header(key, value);
    }
    for (header_name, header_value) in req.headers().iter() {
        if header_name != CONTENT_TYPE && header_name != ACCEPT {
            // builder = builder.header(header_name.as_str(), header_value.to_str()?);
            builder = builder.header(header_name, header_value);
        }
    }

    Ok(builder)
}

async fn send_request(
    mut builder: RequestBuilder,
    req: HttpRequest<BoxBody>,
) -> Result<Response<UnsyncBoxBody<Bytes, ServerError>>, ServerError> {
    log::debug!("BEGIN send_request");

    log::debug!("req: {:?}", &req);
    let body = req.into_body().data().await;
    log::debug!("body: {:?}", &body);
    if let Some(body) = body {
        builder = builder.body(body?);
    }
    let response = builder.send().await?;
    log::debug!("response: {response:?}");

    let mut result = Response::builder().status(response.status());

    for (header_name, header_value) in response.headers().iter() {
        result = result.header(header_name.as_str(), header_value.to_str()?);
    }
    // response
    //     .headers()
    //     .iter()
    //     .map(|(name, value)| result = result.header(name, value));

    let content_type = {
        let content_type_header = match response.headers().get(CONTENT_TYPE) {
            Some(content_type_header) => content_type_header.to_str().map_err(Into::into),
            None => Err(ServerError::MissingContentTypeHeader),
        }?;
        content_type_header.to_owned()
    };

    let bytes = BytesMut::from(response.bytes().await?.as_ref());
    let body = UnsyncBoxBody::new(GrpcResponse::new(bytes, &content_type)?);

    log::debug!("END send_request");
    result.body(body).map_err(Into::into)
}

#[tokio::test]
async fn should_send_request() {
    let builder = reqwest::Client::new().post("http://localhost");
    let body = tonic::body::empty_body();
    let req = http::Request::post("http://localhost")
        .body(body)
        .expect("create http::Request");
    let result = send_request(builder, req).await;
    println!("result: {:?}", result);
    assert!(result.is_ok());
}
