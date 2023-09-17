pub mod types {
    use anyhow::{bail, Result};
    use async_trait::async_trait;
    use chrono::prelude::*;
    use reqwest::Client as HttpClient;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct TwitchData<T> {
        pub data: Vec<T>,
    }

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
    pub struct Client<T: TokenStorage> {
        pub client_id: String,
        pub client_secret: String,
        pub token: Token,
        pub http_client: HttpClient,
        pub token_storage: T,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct User {
        pub id: String,
        pub login: String,
        pub display_name: String,
        pub r#type: String,
        pub broadcaster_type: String,
        pub description: String,
        pub profile_image_url: String,
        pub offline_image_url: String,
        pub view_count: i64,
        pub email: Option<String>,
        pub created_at: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RewardImage {
        pub url_1x: String,
        pub url_2x: String,
        pub url_4x: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RewardMaxPerStream {
        pub is_enabled: bool,
        pub max_per_stream: i64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RewardMaxPerUserPerStream {
        pub is_enabled: bool,
        pub max_per_user_per_stream: i64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RewardGlobalCooldown {
        pub is_enabled: bool,
        pub global_cooldown_seconds: i64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Reward {
        pub broadcaster_id: String,
        pub broadcaster_login: String,
        pub broadcaster_name: String,
        pub id: String,
        pub title: String,
        pub prompt: String,
        pub cost: i64,
        pub image: Option<RewardImage>,
        pub default_image: RewardImage,
        pub background_color: String,
        pub is_enabled: bool,
        pub is_user_input_required: bool,
        pub max_per_stream_setting: RewardMaxPerStream,
        pub max_per_user_per_stream_setting: RewardMaxPerUserPerStream,
        pub global_cooldown_setting: RewardGlobalCooldown,
        pub is_paused: bool,
        pub is_in_stock: bool,
        pub should_redemptions_skip_request_queue: bool,
        pub redemptions_redeemed_current_stream: Option<i64>,
        pub cooldown_expires_at: Option<String>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct RewardCreate {
        pub title: String,
        pub cost: i64,
        pub prompt: Option<String>,
        pub is_enabled: Option<bool>,
        pub background_color: Option<String>,
        pub is_user_input_required: Option<bool>,
        pub is_max_per_stream_enabled: Option<bool>,
        pub max_per_stream: Option<i64>,
        pub is_max_per_user_per_stream_enabled: Option<bool>,
        pub max_per_user_per_stream: Option<i64>,
        pub is_global_cooldown_enabled: Option<bool>,
        pub global_cooldown_seconds: Option<i64>,
        pub should_redemptions_skip_request_queue: Option<bool>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct RedemptionStatus {
        pub status: String,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct EventSubTransport {
        pub method: String,
        pub callback: Option<String>,
        pub secret: Option<String>,
        pub session_id: Option<String>,
        pub connected_at: Option<String>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct EventSubCondition {
        pub broadcaster_user_id: Option<String>,
        pub reward_id: Option<String>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct EventSub {
        pub id: String,
        pub status: String,
        pub r#type: String,
        pub version: String,
        pub condition: EventSubCondition,
        pub created_at: String,
        pub transport: EventSubTransport,
        pub cost: i64,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct EventSubCreate {
        pub r#type: String,
        pub version: String,
        pub condition: EventSubCondition,
        pub transport: EventSubTransport,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct BanUser {
        pub user_id: String,
        pub duration: i64,
        pub reason: Option<String>,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct BanUserObj {
        pub data: BanUser,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct BannedUser {
        pub broadcaster_id: String,
        pub moderator_id: String,
        pub user_id: String,
        pub created_at: String,
        pub end_time: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ChannelInformation {
        pub broadcaster_id: String,
        pub broadcaster_login: String,
        pub broadcaster_name: String,
        pub broadcaster_language: String,
        pub game_name: String,
        pub game_id: String,
        pub title: String,
        pub delay: i64,
        pub tags: Vec<String>,
        pub content_classification_labels: Vec<String>,
        pub is_branded_content: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Whisper {
        pub message: String,
    }

    #[derive(Debug)]
    pub struct VoidStorage {}
    #[async_trait]
    impl TokenStorage for VoidStorage {
        async fn save(&mut self, _token: &Token) -> Result<()> {
            Ok(())
        }
    }

    impl<T: TokenStorage> Client<T> {
        pub async fn http_request<T2: serde::Serialize>(
            &mut self,
            method: reqwest::Method,
            uri: String,
            data_json: Option<T2>,
            data_form: Option<String>,
        ) -> Result<reqwest::Response> {
            let mut req = self.http_client.request(method, uri);

            req = match data_json {
                Some(data_json) => req.json(&data_json),
                None => match data_form {
                    Some(data_form) => req.body(data_form),
                    None => req,
                },
            };

            let req = req
                .timeout(core::time::Duration::from_secs(5))
                .header(
                    "Authorization",
                    format!("Bearer {0}", self.token.access_token),
                )
                .header("Client-Id", self.client_id.clone());

            Ok(req.send().await?)
        }

        pub async fn request<T1: serde::Serialize + std::clone::Clone>(
            &mut self,
            method: reqwest::Method,
            uri: String,
            data_json: Option<T1>,
            data_form: Option<String>,
        ) -> Result<reqwest::Response> {
            let mut res = self
                .http_request(
                    method.clone(),
                    uri.clone(),
                    data_json.clone(),
                    data_form.clone(),
                )
                .await?;

            if res.status() == reqwest::StatusCode::UNAUTHORIZED {
                //Token invalid, get new? If fail, or fail again, return error.
                self.refresh_token().await?;
                res = self.http_request(method, uri, data_json, data_form).await?;
            }

            Ok(res)
        }

        pub async fn request_result<
            T1: for<'de> serde::Deserialize<'de>,
            T2: serde::Serialize + std::clone::Clone,
        >(
            &mut self,
            method: reqwest::Method,
            uri: String,
            data_json: Option<T2>,
            data_form: Option<String>,
        ) -> Result<T1> {
            let res = self
                .request::<T2>(method, uri, data_json, data_form)
                .await?;
            Ok(res.json::<T1>().await?)
        }

        pub async fn get<T1: for<'de> serde::Deserialize<'de>>(
            &mut self,
            uri: String,
        ) -> Result<T1> {
            return self
                .request_result::<T1, String>(reqwest::Method::GET, uri, None, None)
                .await;
        }

        pub async fn post_empty(&mut self, uri: String) -> Result<()> {
            match self
                .request::<String>(reqwest::Method::POST, uri, None, None)
                .await
            {
                Ok(..) => Ok(()),
                Err(e) => Err(e),
            }
        }

        pub async fn post_form<T1: for<'de> serde::Deserialize<'de>>(
            &mut self,
            uri: String,
            data: String,
        ) -> Result<T1> {
            return self
                .request_result::<T1, String>(reqwest::Method::POST, uri, None, Some(data))
                .await;
        }

        pub async fn post_json<
            T1: for<'de> serde::Deserialize<'de>,
            T2: serde::Serialize + std::clone::Clone,
        >(
            &mut self,
            uri: String,
            data: T2,
        ) -> Result<T1> {
            return self
                .request_result::<T1, T2>(reqwest::Method::POST, uri, Some(data), None)
                .await;
        }

        pub async fn post_json_empty<T1: serde::Serialize + std::clone::Clone>(
            &mut self,
            uri: String,
            data: T1,
        ) -> Result<()> {
            match self
                .request::<T1>(reqwest::Method::POST, uri, Some(data), None)
                .await
            {
                Ok(..) => Ok(()),
                Err(e) => Err(e),
            }
        }

        pub async fn patch_json<
            T1: for<'de> serde::Deserialize<'de>,
            T2: serde::Serialize + std::clone::Clone,
        >(
            &mut self,
            uri: String,
            data: T2,
        ) -> Result<T1> {
            return self
                .request_result::<T1, T2>(reqwest::Method::PATCH, uri, Some(data), None)
                .await;
        }

        pub async fn delete(&mut self, uri: String) -> Result<()> {
            self.request::<String>(reqwest::Method::DELETE, uri, None, None)
                .await?;
            Ok(())
        }

        pub async fn get_token_user_id(&mut self) -> Result<String> {
            match &self.token.user {
                Some(v) => Ok(v.id.clone()),
                None => {
                    if self.token.token_type == TokenType::UserAccessToken
                        && self.token.user.is_none()
                    {
                        let user = self.get_user().await?;
                        let id = user.id.clone();
                        self.token.user = Some(user);
                        return Ok(id);
                    }

                    bail!("No User Id");
                }
            }
        }
    }
}

pub mod auth {
    use super::types::*;
    use anyhow::Result;
    use serde::Deserialize;

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
            let res = self
                .http_request::<()>(reqwest::Method::POST, "https://id.twitch.tv/oauth2/token".to_string(), None, Some(format!("client_id={0}&client_secret={1}&grant_type=refresh_token&refresh_token={2}", self.client_id, self.client_secret, self.token.refresh_token)))
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

        pub async fn from_get_app_token(
            client_id: String,
            client_secret: String,
            token_storage: T,
        ) -> Result<Client<T>> {
            let http_client = reqwest::Client::new();
            let token = http_client
                .post("https://id.twitch.tv/oauth2/token")
                .body(format!("client_id={client_id}&client_secret={client_secret}&grant_type=client_credentials"))
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
            client.token.token_type = TokenType::AppAccessToken;
            client.token_storage.save(&client.token).await?;
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
}

pub mod helix {
    use super::types::*;
    use anyhow::{bail, Result};

    impl<T: TokenStorage> Client<T> {
        pub async fn get_users_by_ids(&mut self, user_ids: Vec<i64>) -> Result<Vec<User>> {
            Ok(self
                .get::<TwitchData<User>>(format!(
                    "https://api.twitch.tv/helix/users?id={0}",
                    user_ids
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join("&id=")
                ))
                .await?
                .data)
        }

        pub async fn get_users_by_logins(&mut self, user_logins: Vec<String>) -> Result<Vec<User>> {
            Ok(self
                .get::<TwitchData<User>>(format!(
                    "https://api.twitch.tv/helix/users?login={0}",
                    user_logins.join("&login=")
                ))
                .await?
                .data)
        }

        pub async fn get_user_by_id(&mut self, user_id: i64) -> Result<User> {
            match self.get_users_by_ids(vec![user_id]).await?.first() {
                Some(user) => Ok(user.clone()),
                None => bail!("No User found"),
            }
        }

        pub async fn get_user_by_login(&mut self, user_login: String) -> Result<User> {
            match self.get_users_by_logins(vec![user_login]).await?.first() {
                Some(user) => Ok(user.clone()),
                None => bail!("No User found"),
            }
        }

        pub async fn get_user(&mut self) -> Result<User> {
            match self
                .get::<TwitchData<User>>("https://api.twitch.tv/helix/users".to_string())
                .await?
                .data
                .first()
            {
                Some(user) => Ok(user.clone()),
                None => bail!("No User found"),
            }
        }

        pub async fn create_custom_reward(&mut self, reward: &RewardCreate) -> Result<Reward> {
            let broadcaster_id = self.get_token_user_id().await?;
            match self
                .post_json::<TwitchData<Reward>, _>(format!("https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id={broadcaster_id}"), reward)
                .await?
                .data
                .first()
            {
                Some(reward) => Ok(reward.clone()),
                None => bail!("No User found"),
            }
        }

        pub async fn update_custom_reward(
            &mut self,
            id: String,
            reward: &RewardCreate,
        ) -> Result<Reward> {
            let broadcaster_id = self.get_token_user_id().await?;
            match self
                .patch_json::<TwitchData<Reward>, _>(format!("https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id={broadcaster_id}&id={id}"), reward)
                .await?
                .data
                .first()
            {
                Some(reward) => Ok(reward.clone()),
                None => bail!("No User found"),
            }
        }

        pub async fn get_custom_rewards(&mut self, ids: Vec<String>) -> Result<Vec<Reward>> {
            let broadcaster_id = self.get_token_user_id().await?;
            Ok(self
                .get::<TwitchData<Reward>>(format!(
                    "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id={broadcaster_id}{0}",
                    if ids.len() > 0 { format!("&id={0}", ids.join("&id=") ) } else { "".to_string() }
                ))
                .await?
                .data)
        }

        pub async fn get_custom_reward(&mut self, id: String) -> Result<Reward> {
            match self.get_custom_rewards(vec![id]).await?.first() {
                Some(reward) => Ok(reward.clone()),
                None => bail!("No Reward found"),
            }
        }

        pub async fn delete_custom_reward(&mut self, id: String) -> Result<()> {
            let broadcaster_id = self.get_token_user_id().await?;
            Ok(self
                .delete(format!(
                    "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id={broadcaster_id}&id={id}"
                ))
                .await?)
        }

        pub async fn update_redemptions_status(
            &mut self,
            id: &String,
            redemptions: Vec<String>,
            status: &RedemptionStatus,
        ) -> Result<Vec<RedemptionStatus>> {
            let broadcaster_id = self.get_token_user_id().await?;
            Ok(self
                .patch_json::<TwitchData<RedemptionStatus>, _>(format!(
                    "https://api.twitch.tv/helix/channel_points/custom_rewards/redemptions?broadcaster_id={broadcaster_id}&reward_id={id}{0}",
                    format!("&id={0}", redemptions.join("&id=") )
                ), status)
                .await?
                .data)
        }

        pub async fn update_redemption_status(
            &mut self,
            id: &String,
            redemption: &String,
            status: &RedemptionStatus,
        ) -> Result<RedemptionStatus> {
            match self
                .update_redemptions_status(id, vec![redemption.clone()], status)
                .await?
                .first()
            {
                Some(status) => Ok(status.clone()),
                None => bail!("No Redemption found"),
            }
        }

        pub async fn create_eventsub_subscription(
            &mut self,
            eventsub: &EventSubCreate,
        ) -> Result<EventSub> {
            match self
                .post_json::<TwitchData<EventSub>, _>(
                    format!("https://api.twitch.tv/helix/eventsub/subscriptions"),
                    eventsub,
                )
                .await?
                .data
                .first()
            {
                Some(eventsub) => Ok(eventsub.clone()),
                None => bail!("No EventSub found"),
            }
        }

        pub async fn delete_eventsub_subscription(&mut self, id: String) -> Result<()> {
            Ok(self
                .delete(format!(
                    "https://api.twitch.tv/helix/eventsub/subscriptions?id={id}"
                ))
                .await?)
        }

        pub async fn add_channel_moderator(&mut self, id: String) -> Result<()> {
            let broadcaster_id = self.get_token_user_id().await?;
            Ok(self
                .post_empty(format!(
                    "https://api.twitch.tv/helix/moderation/moderators?broadcaster_id={broadcaster_id}&user_id={id}"
                ))
                .await?)
        }

        pub async fn remove_channel_moderator(&mut self, id: String) -> Result<()> {
            let broadcaster_id = self.get_token_user_id().await?;
            Ok(self
                .delete(format!(
                    "https://api.twitch.tv/helix/moderation/moderators?broadcaster_id={broadcaster_id}&user_id={id}"
                ))
                .await?)
        }

        pub async fn ban_user(
            &mut self,
            broadcaster_id: String,
            banuser: &BanUser,
        ) -> Result<BannedUser> {
            let moderator_id = self.get_token_user_id().await?;
            match self
                .post_json::<TwitchData<BannedUser>, _>(
                    format!("https://api.twitch.tv/helix/moderation/bans?moderator_id={moderator_id}&broadcaster_id={broadcaster_id}"),
                    BanUserObj {
                        data: banuser.clone()
                    },
                )
                .await?
                .data
                .first()
            {
                Some(banneduser) => Ok(banneduser.clone()),
                None => bail!("No EventSub found"),
            }
        }

        pub async fn unban_user(&mut self, broadcaster_id: String, user_id: String) -> Result<()> {
            let moderator_id = self.get_token_user_id().await?;
            Ok(self
                .delete(format!(
                    "https://api.twitch.tv/helix/moderation/bans?moderator_id={moderator_id}&broadcaster_id={broadcaster_id}&user_id={user_id}"
                ))
                .await?)
        }

        pub async fn shoutout(
            &mut self,
            from_broadcaster_id: String,
            to_broadcaster_id: String,
        ) -> Result<()> {
            let moderator_id = self.get_token_user_id().await?;
            Ok(self
                .post_empty(format!(
                    "https://api.twitch.tv/helix/chat/shoutouts?from_broadcaster_id={from_broadcaster_id}&to_broadcaster_id={to_broadcaster_id}&moderator_id={moderator_id}"
                ))
                .await?)
        }

        pub async fn get_channel_information(
            &mut self,
            broadcaster_ids: Vec<String>,
        ) -> Result<Vec<ChannelInformation>> {
            Ok(self
                .get::<TwitchData<ChannelInformation>>(format!(
                    "https://api.twitch.tv/helix/channels?{0}",
                    if broadcaster_ids.len() > 0 {
                        format!(
                            "broadcaster_id={0}",
                            broadcaster_ids.join("&broadcaster_id=")
                        )
                    } else {
                        "".to_string()
                    }
                ))
                .await?
                .data)
        }

        pub async fn whisper(&mut self, to_user_id: String, message: String) -> Result<()> {
            let from_user_id = self.get_token_user_id().await?;
            Ok(self
                .post_json_empty(
                    format!("https://api.twitch.tv/helix/whispers?from_user_id={from_user_id}&to_user_id={to_user_id}"),
                    Whisper {
                        message: message
                    },
                )
                .await?)
        }
    }
}
