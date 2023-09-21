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
}
