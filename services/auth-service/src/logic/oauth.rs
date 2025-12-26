// src/logic/oauth.rs
use std::sync::Arc;
use oauth2::basic::*;
use oauth2::*;
use crate::AppState;

pub async fn create_oauth_client(
    state: &Arc<AppState>,
) -> Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
> {
    let client = BasicClient::new(ClientId::new(state.config.discord_client_id.clone()))
        .set_client_secret(ClientSecret::new(
            state.config.discord_client_secret.clone(),
        ))
        .set_auth_uri(AuthUrl::new(state.config.discord_auth_url.clone()).expect("REASON"))
        .set_token_uri(TokenUrl::new(state.config.discord_token_url.clone()).expect("REASON"))
        .set_redirect_uri(
            RedirectUrl::new(state.config.discord_redirect_uri.clone()).expect("REASON"),
        );
    client
}
