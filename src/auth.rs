use crate::helix::Client;
use crate::helix::User;

use anyhow::Result;
use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

#[async_trait]
pub trait TokenStorage {
    async fn save(&mut self, token: &Token) -> Result<()>;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TokenType {
    #[default]
    UserAccessToken,
    AppAccessToken,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Token {
    #[serde(skip)]
    pub token_type: TokenType,
    #[serde(default)]
    pub refresh_token: String,
    pub access_token: String,
    pub expires_in: i64,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(skip)]
    pub user: Option<User>,
}

#[derive(Debug, Clone)]
pub struct VoidStorage {}
#[async_trait]
impl TokenStorage for VoidStorage {
    async fn save(&mut self, _token: &Token) -> Result<()> {
        Ok(())
    }
}

#[derive(Deserialize)]
struct ValidateToken {
    pub expires_in: i64,
}

impl<T: TokenStorage> Client<T> {
    pub async fn validate_token(&mut self) -> Result<()> {
        let token = match self
            .get::<ValidateToken>("https://id.twitch.tv/oauth2/validate".to_string())
            .await
        {
            Ok(r) => r,
            Err(..) => {
                self.refresh_token().await?;
                return Ok(());
            }
        };

        if token.expires_in < 3600 {
            self.refresh_token().await?;
        }

        Ok(())
    }

    pub async fn refresh_token(&mut self) -> Result<()> {
        if self.token.token_type == TokenType::AppAccessToken {
            self.get_app_token().await?;
            return Ok(());
        }

        let res = self
            .http_request::<()>(
                reqwest::Method::POST,
                "https://id.twitch.tv/oauth2/token".to_string(),
                None,
                Some(format!(
                    "client_id={0}&client_secret={1}&grant_type=refresh_token&refresh_token={2}",
                    self.client_id, self.client_secret, self.token.refresh_token
                )),
            )
            .await?;

        self.token = res.json::<Token>().await?;
        self.token_storage.save(&self.token).await?;

        Ok(())
    }

    pub fn from_token_no_validation(
        client_id: String,
        client_secret: String,
        token_storage: T,
        token: Token,
    ) -> Client<T> {
        Client {
            client_id: client_id,
            client_secret: client_secret,
            token: token,
            http_client: reqwest::Client::new(),
            token_storage: token_storage,
        }
    }

    pub async fn from_token(
        client_id: String,
        client_secret: String,
        token_storage: T,
        token: Token,
    ) -> Result<Client<T>> {
        let mut client =
            Self::from_token_no_validation(client_id, client_secret, token_storage, token);
        client.token.user = Some(client.get_user().await?);
        Ok(client)
    }

    async fn get_app_token(&mut self) -> Result<()> {
        let token = self
            .http_client
            .post("https://id.twitch.tv/oauth2/token")
            .body(format!(
                "client_id={0}&client_secret={1}&grant_type=client_credentials",
                self.client_id, self.client_secret
            ))
            .send()
            .await?
            .json::<Token>()
            .await?;

        self.token = token;
        self.token.token_type = TokenType::AppAccessToken;
        self.token_storage.save(&self.token).await?;

        Ok(())
    }

    pub async fn from_get_app_token(
        client_id: String,
        client_secret: String,
        token_storage: T,
    ) -> Result<Client<T>> {
        let http_client = reqwest::Client::new();
        let mut client = Client {
            client_id: client_id,
            client_secret: client_secret,
            http_client: http_client,
            token_storage: token_storage,
            token: Token::default(),
        };
        client.get_app_token().await?;
        Ok(client)
    }

    pub async fn from_authorization(
        client_id: String,
        client_secret: String,
        token_storage: T,
        code: String,
        redirect_uri: String,
    ) -> Result<Client<T>> {
        let http_client = reqwest::Client::new();
        let token = http_client.post("https://id.twitch.tv/oauth2/token")
                .body(format!("client_id={client_id}&client_secret={client_secret}&code={code}&grant_type=authorization_code&redirect_uri={redirect_uri}"))
                .send()
                .await?
                .json::<Token>()
                .await?;
        let mut client = Client {
            client_id: client_id,
            client_secret: client_secret,
            token: token,
            http_client: http_client,
            token_storage: token_storage,
        };
        client.token.user = Some(client.get_user().await?);
        client.token_storage.save(&client.token).await?;
        Ok(client)
    }
}
