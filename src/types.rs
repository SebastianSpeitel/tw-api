use anyhow::Result;
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
pub struct Announcement {
    pub message: String,
    pub color: Option<String>,
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
