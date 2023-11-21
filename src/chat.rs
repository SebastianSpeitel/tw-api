use anyhow::bail;
use anyhow::Result;

use irc::client::data::Config as IrcConfig;
use irc::client::prelude::Capability as IrcCap;
use irc::client::Client as IrcClient;
use irc::client::ClientStream as IrcStream;

impl<T: crate::auth::TokenStorage> crate::helix::Client<T> {
    pub async fn connect_chat(&mut self, channels: Vec<String>) -> Result<(IrcClient, IrcStream)> {
        match self.validate_token().await {
            Err(e) => {
                println!("{e:?}");
                bail!("Invalid refresh token or no internet");
            }
            _ => {}
        };

        let channels = channels
            .into_iter()
            .map(|c| {
                format!(
                    "{0}{1}",
                    if c.starts_with("#") { "" } else { "#" },
                    c.to_lowercase()
                )
            })
            .collect();

        let config = IrcConfig {
            server: Some("irc.chat.twitch.tv".to_owned()),
            port: Some(6697),
            use_tls: Some(true),
            nickname: Some(self.get_token_user_login().await?.to_lowercase().to_owned()),
            password: Some(format!("oauth:{0}", self.token.access_token)),
            channels: channels,
            ..Default::default()
        };

        let mut client = match IrcClient::from_config(config).await {
            Ok(v) => v,
            Err(e) => {
                println!("{e:?}");
                bail!("IrcClient::from_config failed");
            }
        };
        match client.send_cap_req(&[
            IrcCap::Custom("twitch.tv/tags"),
            IrcCap::Custom("twitch.tv/commands"),
        ]) {
            Err(e) => {
                println!("{e:?}");
                bail!("IrcClient.send_cap_req failed");
            }
            _ => {}
        };
        match client.identify() {
            Err(e) => {
                println!("{e:?}");
                bail!("IrcClient.identify failed");
            }
            _ => {}
        };

        let stream = match client.stream() {
            Ok(v) => v,
            Err(e) => {
                println!("{e:?}");
                bail!("IrcClient.stream failed");
            }
        };

        Ok((client, stream))
    }
}
