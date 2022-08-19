use serde::{Serialize, Deserialize};
use crate::{CLIENT, DiscordResult, map_reqwest_error_status, path};

#[derive(Debug, Clone, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
}

pub async fn access_token_exchange(
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> DiscordResult<AccessToken> {
    #[derive(Serialize)]
    struct Request<'a> {
        client_id: &'a str,
        client_secret: &'a str,
        grant_type: &'a str,
        code: &'a str,
        redirect_uri: &'a str,
    }

    let resp: AccessToken = CLIENT.post(path("/oauth2/token"))
        .form(&Request {
            client_id,
            client_secret,
            grant_type: "authorization_code",
            code,
            redirect_uri,
        })
        .send()
        .await?
        .error_for_status()
        .map_err(map_reqwest_error_status)?
        .json()
        .await?;

    Ok(resp)
}

pub async fn refresh_access_token(
    client_id: &str,
    client_secret: &str,
    refresh_token: &str
) -> DiscordResult<AccessToken> {
    #[derive(Serialize)]
    struct Request<'a> {
        client_id: &'a str,
        client_secret: &'a str,
        refresh_token: &'a str,
    }

    let resp: AccessToken = CLIENT.post(path("/oauth2/token"))
        .form(&Request {
            client_id,
            client_secret,
            refresh_token
        })
        .send()
        .await?
        .error_for_status()
        .map_err(map_reqwest_error_status)?
        .json()
        .await?;
    Ok(resp)
}