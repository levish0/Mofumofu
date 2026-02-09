use crate::config::db_config::DbConfig;
use crate::dto::oauth::internal::github::{GithubEmail, GithubUserInfo};
use crate::service::error::errors::{Errors, ServiceResult};
use crate::service::oauth::provider::common::{build_oauth_client, exchange_oauth_code};
use oauth2::basic::{
    BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse,
};
use oauth2::{AccessToken, Client, EndpointNotSet, EndpointSet, StandardRevocableToken};
use reqwest::Client as ReqwestClient;

const GITHUB_AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const GITHUB_USERINFO_URL: &str = "https://api.github.com/user";
const GITHUB_EMAILS_URL: &str = "https://api.github.com/user/emails";
const GITHUB_USER_AGENT: &str = "mofumofu/1.0";

fn build_github_client() -> Result<
    Client<
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
        &config.github_client_id,
        &config.github_client_secret,
        &config.github_redirect_uri,
        GITHUB_AUTH_URL,
        GITHUB_TOKEN_URL,
    )
}

pub async fn exchange_github_code(code: &str) -> ServiceResult<AccessToken> {
    let client = build_github_client()?;
    exchange_oauth_code(client, code, "GitHub").await
}

pub async fn get_github_user_info(
    http_client: &ReqwestClient,
    access_token: &AccessToken,
) -> ServiceResult<GithubUserInfo> {
    let response = http_client
        .get(GITHUB_USERINFO_URL)
        .bearer_auth(access_token.secret())
        .header("User-Agent", GITHUB_USER_AGENT)
        .send()
        .await
        .map_err(|_e| Errors::OauthUserInfoFetchFailed)?;

    let mut user_info: GithubUserInfo = response
        .json()
        .await
        .map_err(|_e| Errors::OauthUserInfoParseFailed)?;

    if user_info.email.is_none() {
        let email_response = http_client
            .get(GITHUB_EMAILS_URL)
            .bearer_auth(access_token.secret())
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await;

        if let Ok(response) = email_response {
            if response.status().is_success() {
                if let Ok(emails) = response.json::<Vec<GithubEmail>>().await {
                    // primary 이메일 찾기
                    if let Some(primary_email) = emails.iter().find(|e| e.primary && e.verified) {
                        user_info.email = Some(primary_email.email.clone());
                    }
                }
            }
        }
    }

    Ok(user_info)
}
