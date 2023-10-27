use chrono::{Duration, Utc};
use dotenv_codegen::dotenv;
use oauth2::Scope;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubPlan {
    pub name: String,
    pub space: u64,
    pub collaborators: u64,
    pub private_repos: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubIdentity {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub usertype: String,
    pub site_admin: bool,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: u64,
    pub public_gists: u64,
    pub followers: u64,
    pub following: u64,
    pub created_at: String,
    pub updated_at: String,
    pub private_gists: Option<u64>,
    pub total_private_repos: u64,
    pub owned_private_repos: u64,
    pub disk_usage: u64,
    pub collaborators: u64,
    pub two_factor_authentication: bool,
    pub plan: GithubPlan,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordIdentity {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<i64>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<i64>,
    pub premium_type: Option<i64>,
    pub public_flags: Option<i64>,
    pub avatar_decoration: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserType {
    Github(GithubIdentity),
    Discord(DiscordIdentity),
}

impl From<DiscordIdentity> for UserType {
    fn from(value: DiscordIdentity) -> Self {
        Self::Discord(value)
    }
}

impl From<GithubIdentity> for UserType {
    fn from(value: GithubIdentity) -> Self {
        Self::Github(value)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JWT {
    pub user: UserType,
    pub iat: i64,
    pub exp: i64,
}

impl From<UserType> for JWT {
    fn from(value: UserType) -> Self {
        let current_time = Utc::now();
        let expiration_time = current_time + Duration::days(30);

        JWT {
            user: value,
            iat: current_time.timestamp(),
            exp: expiration_time.timestamp(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OAuth2Info {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
}

impl OAuth2Info {
    pub fn github_default() -> Self {
        let client_id = dotenv!("GITHUB_CLIENT_ID");
        let client_secret = dotenv!("GITHUB_CLIENT_SECRET");
        let auth_url = "https://github.com/login/oauth/authorize".to_string();
        let token_url = "https://github.com/login/oauth/access_token".to_string();
        OAuth2Info {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            auth_url,
            token_url,
            redirect_url: "http://localhost:8080/oauth_callback".to_string(),
        }
    }
    pub fn discord_default() -> Self {
        let client_id = dotenv!("DISCORD_CLIENT_ID");
        let client_secret = dotenv!("DISCORD_CLIENT_SECRET");
        let auth_url = "https://discord.com/api/oauth2/authorize".to_string();
        let token_url = "https://discord.com/api/oauth2/token".to_string();

        OAuth2Info {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            auth_url,
            token_url,
            redirect_url: "http://localhost:49103/oauth/callback/discord".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum OAuthType {
    Standard,
    Github,
}

#[derive(Clone, Debug)]
pub struct OAuthParams {
    pub provider_name: String,
    pub general_info: OAuth2Info,
    pub scopes: Vec<Scope>,
    pub oauth_type: OAuthType,
}

impl OAuthParams {
    pub fn discord_default() -> Self {
        OAuthParams {
            provider_name: "Discord".to_string(),
            general_info: OAuth2Info::discord_default(),
            scopes: vec![Scope::new("identify".to_string())],
            oauth_type: OAuthType::Standard,
        }
    }
    pub fn github_default() -> Self {
        OAuthParams {
            provider_name: "Github".to_string(),
            general_info: OAuth2Info::github_default(),
            scopes: vec![],
            oauth_type: OAuthType::Github,
        }
    }
}
