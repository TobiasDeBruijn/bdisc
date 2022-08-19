use actix_multiresponse::Payload;
use dal::mysql::Oauth2State;
use crate::data::WebData;
use crate::error::WebResult;
use crate::token::Token;
use proto::PostOauth2LoginResponse;
use urlencoding::encode;

pub async fn login(data: WebData, token: Token) -> WebResult<Payload<PostOauth2LoginResponse>> {
    let state = Oauth2State::new(data.mysql.clone(), token.token.clone()).await?;

    let scopes = vec![
        "gdm.join",
        "guilds",
        "guilds.members.read",
        "identify",
        "messages.read",
    ].join(" ");

    let url = format!("https://discord.com/api/oauth2/authorize?response_type={response_type}&client_id={client_id}&scope={scope}&state={state}&redirect_uri={redirect_uri}",
        response_type = "code",
        client_id = &data.config.discord.client_id,
        scope = encode(&scopes),
        state = &state.state,
        redirect_uri = encode(&format!("{}/oauth2/callback", &data.config.server.base_url)),
    );

    Ok(Payload(PostOauth2LoginResponse {
        redirect_url: url,
    }))
}