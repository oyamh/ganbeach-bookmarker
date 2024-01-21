// use serde::{Deserialize, Serialize};
use serde::Serialize;

//JsValue(Object({"name":"refresh-token","value":"","domain":"localhost","hostOnly":true,"path":"/","secure":false,"httpOnly":true,"sameSite":"no_restriction","session":false,"firstPartyDomain":"","partitionKey":null,"expirationDate":1668645034,"storeId":"firefox-default"}))

#[derive(Serialize)]
pub struct CookieDetails {
    pub name: &'static str,
    pub url: &'static str,
}
