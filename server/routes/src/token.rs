use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use dal::mysql::ApiToken;
use crate::data::WebData;
use crate::error::WebError;

pub struct Token(ApiToken);

impl Deref for Token {
    type Target = ApiToken;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for Token {
    type Error = WebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let token = req.headers()
                .get("authorization")
                .ok_or(WebError::Unauthorized)?
                .to_str()
                .map_err(|_| WebError::Unauthorized)?;

            let data: &WebData = req.app_data().unwrap();
            let api_token = ApiToken::get(data.mysql.clone(), token)
                .await?
                .ok_or(WebError::Unauthorized)?;

            Ok(Self(api_token))
        })
    }
}