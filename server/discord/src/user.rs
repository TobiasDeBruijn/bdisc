use proto::User;
use crate::{CLIENT, DiscordResult, map_reqwest_error_status, path};

pub async fn get_current(access_token: &str) -> DiscordResult<User> {
    let resp: User = CLIENT.get(path("/users/@me"))
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await?
        .error_for_status()
        .map_err(map_reqwest_error_status)?
        .json()
        .await?;
    Ok(resp)
}