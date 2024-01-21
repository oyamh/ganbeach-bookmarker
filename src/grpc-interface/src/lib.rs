mod account;
mod add;
mod client;
mod convert;
mod error;
mod get;
mod grpc_response;

pub(crate) mod gooscut {
    tonic::include_proto!("gooscut");
}

pub use account::*;
pub use add::*;
// pub use data_type::*;
pub use error::ServerError;
pub use get::*;
pub use grpc_response::*;
// pub use request::*;
// pub use response::*;

const GRPC_HTTP_HEADER: &'static str = "application/grpc-web+proto";
const GRPC_TEXT_HEADER: &'static str = "application/grpc-web-text+proto";
