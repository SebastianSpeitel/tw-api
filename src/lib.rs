pub mod auth;
pub mod eventsub;
pub mod helix;
pub mod types;

use anyhow::{bail, Result};
use reqwest::{Method, Response};
use types::{Client, TokenStorage, TokenType};

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

    pub async fn get_token_user_id(&mut self) -> Result<String> {
        match &self.token.user {
            Some(v) => Ok(v.id.clone()),
            None => {
                if self.token.token_type == TokenType::UserAccessToken && self.token.user.is_none()
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
