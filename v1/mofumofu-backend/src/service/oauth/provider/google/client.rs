use crate::config::db_config::DbConfig;
use crate::dto::oauth::internal::google::GoogleUserInfo;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::service::oauth::provider::common::{build_oauth_client, exchange_oauth_code};
use oauth2::basic::{
    BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse,
};
use oauth2::{
    AccessToken, Client as OauthClient, EndpointNotSet, EndpointSet, StandardRevocableToken,
};
use reqwest::Client as ReqwestClient;

const GOOGLE_USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v3/userinfo";
const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

fn build_google_client() -> Result<
    OauthClient<
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
    >,
    Errors,
> {
    let config = DbConfig::get();
    build_oauth_client(
        &config.google_client_id,
        &config.google_client_secret,
        &config.google_redirect_uri,
        GOOGLE_AUTH_URL,
        GOOGLE_TOKEN_URL,
    )
}

fn build_google_link_client() -> Result<
    OauthClient<
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
    >,
    Errors,
> {
    let config = DbConfig::get();
    build_oauth_client(
        &config.google_client_id,
        &config.google_client_secret,
        &config.google_link_redirect_uri,
        GOOGLE_AUTH_URL,
        GOOGLE_TOKEN_URL,
    )
}

pub async fn exchange_google_code(code: &str) -> ServiceResult<AccessToken> {
    let client = build_google_client()?;
    exchange_oauth_code(client, code, "Google").await
}

pub async fn exchange_google_code_for_linking(code: &str) -> ServiceResult<AccessToken> {
    let client = build_google_link_client()?;
    exchange_oauth_code(client, code, "Google").await
}

pub async fn get_google_user_info(
    http_client: &ReqwestClient,
    access_token: &AccessToken,
) -> ServiceResult<GoogleUserInfo> {
    let response = http_client
        .get(GOOGLE_USERINFO_URL)
        .bearer_auth(access_token.secret())
        .send()
        .await
        .map_err(|_e| Errors::OauthUserInfoFetchFailed)?;

    if !response.status().is_success() {
        return Err(Errors::OauthUserInfoFetchFailed);
    }

    let user_info: GoogleUserInfo = response
        .json()
        .await
        .map_err(|_e| Errors::OauthUserInfoParseFailed)?;

    Ok(user_info)
}
