use actix_web::cookie::time;
use actix_web::web;
use crate::data::WebData;
use serde::Deserialize;
use dal::mysql::{Oauth2State, User};
use crate::empty::Empty;
use crate::error::{WebError, WebResult};

#[derive(Debug, Deserialize)]
pub struct Query {
    code: String,
    state: String,
}

pub async fn callback(data: WebData, query: web::Query<Query>) -> WebResult<Empty> {
    Oauth2State::get(data.mysql.clone(), &query.state)
        .await?
        .ok_or(WebError::NotFound("State not found".to_string()))?;

    let instant = time::Instant::now();
    let access_token = discord::oauth2::access_token_exchange(
        &data.config.discord.client_id,
        &data.config.discord.client_secret,
        &query.code,
        &format!("{}/oauth2/callback", &data.config.server.base_url)
    ).await?;

    let discord_user = discord::user::get_current(&access_token.access_token).await?;
    let user = User::new(data.mysql.clone(), discord_user.id, access_token.refresh_token).await?;
    user.add_access_token(
        access_token.access_token,
        time::OffsetDateTime::now_utc().unix_timestamp() + access_token.expires_in - instant.elapsed().whole_seconds()
    ).await?;

    Ok(Empty)
}