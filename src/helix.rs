use crate::auth::{Token, TokenStorage, TokenType};
use anyhow::bail;
use anyhow::Result;

use reqwest::Client as HttpClient;
use reqwest::{Method, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TwitchData<T> {
    pub data: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub cursor: Option<String>,
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
pub struct RewardUpdate {
    pub title: Option<String>,
    pub cost: Option<i64>,
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
    pub is_paused: Option<bool>,
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
    pub broadcaster_id: Option<String>,
    pub broadcaster_user_id: Option<String>,
    pub moderator_user_id: Option<String>,
    pub user_id: Option<String>,
    pub from_broadcaster_user_id: Option<String>,
    pub to_broadcaster_user_id: Option<String>,
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
pub struct PredictionTopPredictor {
    pub user_id: String,
    pub user_name: String,
    pub user_login: String,
    pub channel_points_used: i64,
    pub channel_points_won: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionOutcome {
    pub id: String,
    pub title: String,
    pub users: i64,
    pub channel_points: i64,
    pub top_predictors: Option<Vec<PredictionTopPredictor>>,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub id: String,
    pub broadcaster_id: String,
    pub broadcaster_name: String,
    pub broadcaster_login: String,
    pub title: String,
    pub winning_outcome_id: Option<String>,
    pub outcomes: Vec<PredictionOutcome>,
    pub prediction_window: i64,
    pub status: String,
    pub created_at: String,
    pub ended_at: Option<String>,
    pub locked_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionOutcomeCreate {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionCreate {
    pub broadcaster_id: String,
    pub title: String,
    pub outcomes: Vec<PredictionOutcomeCreate>,
    pub prediction_window: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionEnd {
    pub broadcaster_id: String,
    pub id: String,
    pub status: String,
    pub winning_outcome_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommercialStart {
    pub broadcaster_id: String,
    pub length: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commercial {
    pub length: i64,
    pub message: String,
    pub retry_after: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub message: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Whisper {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    pub id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub game_id: String,
    pub game_name: String,
    pub r#type: String,
    pub title: String,
    pub tags: Vec<String>,
    pub viewer_count: i64,
    pub started_at: String,
    pub language: String,
    pub thumbnail_url: String,
    pub is_mature: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelFollowers {
    pub followed_at: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelFollowersData {
    pub data: Vec<ChannelFollowers>,
    pub pagination: Pagination,
    pub total: i64,
}

impl<T: TokenStorage> Client<T> {
    pub async fn http_request<T2: serde::Serialize>(
        &mut self,
        method: Method,
        uri: String,
        data_json: Option<T2>,
        data_form: Option<String>,
    ) -> Result<Response> {
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
        method: Method,
        uri: String,
        data_json: Option<T1>,
        data_form: Option<String>,
    ) -> Result<Response> {
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
        method: Method,
        uri: String,
        data_json: Option<T2>,
        data_form: Option<String>,
    ) -> Result<T1> {
        let res = self
            .request::<T2>(method, uri, data_json, data_form)
            .await?;
        Ok(res.json::<T1>().await?)
    }

    pub async fn get<T1: for<'de> serde::Deserialize<'de>>(&mut self, uri: String) -> Result<T1> {
        return self
            .request_result::<T1, String>(Method::GET, uri, None, None)
            .await;
    }

    pub async fn post_empty(&mut self, uri: String) -> Result<()> {
        match self.request::<String>(Method::POST, uri, None, None).await {
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
            .request_result::<T1, String>(Method::POST, uri, None, Some(data))
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
            .request_result::<T1, T2>(Method::POST, uri, Some(data), None)
            .await;
    }

    pub async fn post_json_empty<T1: serde::Serialize + std::clone::Clone>(
        &mut self,
        uri: String,
        data: T1,
    ) -> Result<()> {
        match self
            .request::<T1>(Method::POST, uri, Some(data), None)
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
            .request_result::<T1, T2>(Method::PATCH, uri, Some(data), None)
            .await;
    }

    pub async fn delete(&mut self, uri: String) -> Result<()> {
        self.request::<String>(Method::DELETE, uri, None, None)
            .await?;
        Ok(())
    }

    pub async fn get_token_user(&mut self) -> Result<User> {
        match &self.token.user {
            Some(v) => Ok(v.clone()),
            None => {
                if self.token.token_type == TokenType::UserAccessToken && self.token.user.is_none()
                {
                    let user = self.get_user().await?;
                    self.token.user = Some(user.clone());
                    return Ok(user);
                }

                bail!("No User");
            }
        }
    }

    pub async fn get_token_user_id(&mut self) -> Result<String> {
        match &self.token.user {
            Some(v) => Ok(v.id.clone()),
            None => {
                let user = self.get_token_user().await?;
                return Ok(user.id.clone());
            }
        }
    }

    pub async fn get_token_user_login(&mut self) -> Result<String> {
        match &self.token.user {
            Some(v) => Ok(v.id.clone()),
            None => {
                let user = self.get_token_user().await?;
                return Ok(user.login.clone());
            }
        }
    }

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
        reward: &RewardUpdate,
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
                None => bail!("Ban User failed"),
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

    pub async fn get_predictions(
        &mut self,
        id: Option<String>,
        first: Option<String>,
        after: Option<String>,
    ) -> Result<Vec<Prediction>> {
        let broadcaster_id = self.get_token_user_id().await?;
        Ok(self
            .get::<TwitchData<Prediction>>(format!(
                "https://api.twitch.tv/helix/predictions?broadcaster_id={broadcaster_id}{0}{1}{2}",
                if let Some(id) = id {
                    format!("&id={id}")
                } else {
                    "".to_string()
                },
                if let Some(first) = first {
                    format!("&first={first}")
                } else {
                    "".to_string()
                },
                if let Some(after) = after {
                    format!("&after={after}")
                } else {
                    "".to_string()
                },
            ))
            .await?
            .data)
    }

    pub async fn create_prediction(
        &mut self,
        title: String,
        outcomes: Vec<String>,
        prediction_window: i64,
    ) -> Result<Prediction> {
        let broadcaster_id = self.get_token_user_id().await?;
        match self
            .post_json::<TwitchData<Prediction>, _>(
                "https://api.twitch.tv/helix/predictions".to_string(),
                PredictionCreate {
                    broadcaster_id: broadcaster_id,
                    title: title,
                    outcomes: outcomes
                        .into_iter()
                        .map(|o| PredictionOutcomeCreate { title: o })
                        .collect(),
                    prediction_window: prediction_window,
                },
            )
            .await?
            .data
            .first()
        {
            Some(prediction) => Ok(prediction.clone()),
            None => bail!("Create Prediction failed"),
        }
    }

    pub async fn end_prediction(
        &mut self,
        id: String,
        status: String,
        winning_outcome_id: Option<String>,
    ) -> Result<Prediction> {
        let broadcaster_id = self.get_token_user_id().await?;
        match self
            .patch_json::<TwitchData<Prediction>, _>(
                "https://api.twitch.tv/helix/predictions".to_string(),
                PredictionEnd {
                    broadcaster_id: broadcaster_id,
                    id: id,
                    status: status,
                    winning_outcome_id: winning_outcome_id,
                },
            )
            .await?
            .data
            .first()
        {
            Some(prediction) => Ok(prediction.clone()),
            None => bail!("End Prediction failed"),
        }
    }

    pub async fn send_chat_announcement(
        &mut self,
        broadcaster_id: String,
        message: String,
        color: Option<String>,
    ) -> Result<()> {
        let moderator_id = self.get_token_user_id().await?;
        Ok(self
                .post_json_empty(
                    format!("https://api.twitch.tv/helix/chat/announcements?broadcaster_id={broadcaster_id}&moderator_id={moderator_id}"),
                    Announcement {
                        message: message,
                        color: color,
                    }
                )
                .await?)
    }

    pub async fn start_commercial(&mut self, length: i64) -> Result<Commercial> {
        let broadcaster_id = self.get_token_user_id().await?;
        match self
            .post_json::<TwitchData<Commercial>, _>(
                "https://api.twitch.tv/helix/channels/commercial".to_string(),
                CommercialStart {
                    broadcaster_id: broadcaster_id,
                    length: length,
                },
            )
            .await?
            .data
            .first()
        {
            Some(commercial) => Ok(commercial.clone()),
            None => bail!("Start Commercial failed"),
        }
    }

    pub async fn get_streams(
        &mut self,
        user_ids: Option<Vec<String>>,
        user_logins: Option<Vec<String>>,
        game_ids: Option<Vec<String>>,
        r#type: Option<String>,
        languages: Option<Vec<String>>,
        first: Option<i64>,
        before: Option<String>,
        after: Option<String>,
    ) -> Result<Vec<Stream>> {
        Ok(self
            .get::<TwitchData<Stream>>(format!(
                "https://api.twitch.tv/helix/streams?{0}{1}{2}{3}{4}{5}{6}{7}",
                if let Some(user_ids) = user_ids {
                    format!("&user_id={}", user_ids.join("&user_id="))
                } else {
                    "".to_string()
                },
                if let Some(user_logins) = user_logins {
                    format!("&user_login={}", user_logins.join("&user_login="))
                } else {
                    "".to_string()
                },
                if let Some(game_ids) = game_ids {
                    format!("&game_id={}", game_ids.join("&game_id="))
                } else {
                    "".to_string()
                },
                if let Some(type_) = r#type {
                    format!("&type={type_}")
                } else {
                    "".to_string()
                },
                if let Some(languages) = languages {
                    format!("&language={}", languages.join("&language="))
                } else {
                    "".to_string()
                },
                if let Some(first) = first {
                    format!("&first={first}")
                } else {
                    "".to_string()
                },
                if let Some(before) = before {
                    format!("&before={before}")
                } else {
                    "".to_string()
                },
                if let Some(after) = after {
                    format!("&after={after}")
                } else {
                    "".to_string()
                },
            ))
            .await?
            .data)
    }

    pub async fn get_stream(&mut self) -> Result<Stream> {
        let broadcaster_id = self.get_token_user_id().await?;
        match self
            .get_streams(
                vec![broadcaster_id].into(),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await?
            .first()
        {
            Some(stream) => Ok(stream.clone()),
            None => bail!("No stream found"),
        }
    }

    pub async fn add_channel_vip(&mut self, id: String) -> Result<()> {
        let broadcaster_id = self.get_token_user_id().await?;
        Ok(self
                .post_empty(format!(
                    "https://api.twitch.tv/helix/channels/vips?broadcaster_id={broadcaster_id}&user_id={id}"
                ))
                .await?)
    }

    pub async fn remove_channel_vip(&mut self, id: String) -> Result<()> {
        let broadcaster_id = self.get_token_user_id().await?;
        Ok(self
                .delete(format!(
                    "https://api.twitch.tv/helix/channels/vips?broadcaster_id={broadcaster_id}&user_id={id}"
                ))
                .await?)
    }

    pub async fn get_channel_followers_total(&mut self, broadcaster_id: String) -> Result<i64> {
        Ok(self
            .get::<ChannelFollowersData>(format!(
                "https://api.twitch.tv/helix/channels/followers?broadcaster_id={0}",
                broadcaster_id
            ))
            .await?
            .total)
    }
}
