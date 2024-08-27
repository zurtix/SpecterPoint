use common::{
    error::{Error, Result},
    models::user::BaseCredential,
};
use reqwest::{cookie::Jar, StatusCode};
use serde_json::Value;
use sqlx::types::Json;
use std::sync::Arc;

pub struct ClientBuilder {
    creds: BaseCredential,
    server: String,
    proxy: Option<String>,
}

impl ClientBuilder {
    pub fn new(username: String, password: String, server: String) -> Self {
        Self {
            creds: BaseCredential { username, password },
            server,
            proxy: None,
        }
    }

    pub fn proxy(&mut self, proxy: String) {
        self.proxy = Some(proxy)
    }

    pub async fn build(self) -> Client {
        let cookie_jar = Arc::new(Jar::default());
        let mut builder = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .cookie_provider(cookie_jar.clone());

        if let Some(p) = self.proxy {
            builder = builder.proxy(reqwest::Proxy::http(p).unwrap());
        }

        Client {
            creds: self.creds,
            server: self.server,
            client: builder.build().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct ApiResponse<T> {
    pub status: StatusCode,
    pub json: Option<Json<T>>,
}

pub struct Client {
    server: String,
    creds: BaseCredential,
    client: reqwest::Client,
}

async fn handle_response<T>(response: reqwest::Response) -> Result<ApiResponse<T>>
where
    T: for<'a> serde::de::Deserialize<'a>,
{
    if response.status().is_success() {
        Ok(ApiResponse {
            status: response.status(),
            json: Some(response.json().await?),
        })
    } else {
        Ok(ApiResponse {
            status: response.status(),
            json: None,
        })
    }
}

impl Client {
    async fn authenticate(&self) -> Result<()> {
        let body = serde_json::to_string(&self.creds).map_err(|_| Error::Auth)?;
        let res = self
            .client
            .post(format!("{}/login", self.server))
            .json(&self.creds)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(Error::Auth);
        }

        Ok(())
    }

    pub async fn request<T>(
        self,
        method: reqwest::Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<ApiResponse<T>>
    where
        T: for<'a> serde::de::Deserialize<'a>,
    {
        self.authenticate().await?;

        let result = match method {
            reqwest::Method::GET => {
                let res = self
                    .client
                    .get(format!("{}/api{}", self.server, path))
                    .send()
                    .await?;

                handle_response::<T>(res).await?
            }
            reqwest::Method::POST => {
                if let Some(bdy) = body {
                    let res = self
                        .client
                        .post(format!("{}/api{}", self.server, path))
                        .json(&bdy)
                        .send()
                        .await?;

                    handle_response::<T>(res).await?
                } else {
                    let res = self
                        .client
                        .post(format!("{}/api{}", self.server, path))
                        .send()
                        .await?;

                    handle_response::<T>(res).await?
                }
            }
            reqwest::Method::DELETE => {
                let res = self
                    .client
                    .delete(format!("{}/api{}", self.server, path))
                    .send()
                    .await?;

                handle_response::<T>(res).await?
            }
            _ => ApiResponse {
                status: StatusCode::BAD_REQUEST,
                json: None,
            },
        };

        println!("{}", result.status);
        Ok(result)
    }
}
