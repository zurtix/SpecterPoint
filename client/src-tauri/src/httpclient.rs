use common::{
    error::{Error, Result},
    models::user::BaseCredential,
};
use reqwest::{cookie::Jar, StatusCode};
use sqlx::types::Json;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::Mutex;

pub struct RefreshMetadata {
    creds: BaseCredential,
    expires_at: SystemTime,
}

#[derive(Debug, Default)]
pub struct ApiResponse<T = serde_json::Value> {
    pub status: StatusCode,
    pub json: Option<Json<T>>,
}

pub struct ClientBuilder {
    server: String,
    metadata: Option<RefreshMetadata>,
    proxy: Option<String>,
}

impl ClientBuilder {
    pub fn new<T: std::fmt::Display>(server: &T) -> Self {
        Self {
            server: server.to_string(),
            metadata: None,
            proxy: None,
        }
    }

    pub fn auth(mut self, username: String, password: String) -> Self {
        self.metadata = Some(RefreshMetadata {
            creds: BaseCredential { username, password },
            expires_at: SystemTime::now(),
        });
        self
    }

    pub fn proxy(mut self, proxy: String) -> Self {
        self.proxy = Some(proxy);
        self
    }

    pub fn build(self) -> Result<Client> {
        let cookie_jar = Arc::new(Jar::default());
        let mut builder = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .cookie_provider(cookie_jar.clone());

        match self.metadata {
            Some(metadata) => {
                if let Some(p) = self.proxy {
                    builder = builder.proxy(reqwest::Proxy::http(p)?);
                }

                Ok(Client::new(metadata, self.server, builder.build()?))
            }
            _ => Err(Error::Auth),
        }
    }
}

pub struct Client {
    metadata: Mutex<RefreshMetadata>,
    server: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(metadata: RefreshMetadata, server: String, client: reqwest::Client) -> Self {
        Self {
            metadata: Mutex::new(metadata),
            server,
            client,
        }
    }

    async fn authenticate(&self) -> Result<()> {
        if self.metadata.lock().await.expires_at <= SystemTime::now() {
            let res = self
                .client
                .post(format!("{}/login", self.server))
                .json(&self.metadata.lock().await.creds)
                .send()
                .await?;

            if res.status().is_success() {
                let max_age_str = res
                    .headers()
                    .get("set-cookie")
                    .and_then(|header| Some(header.to_str().unwrap_or("")))
                    .and_then(|s| {
                        s.split(';').find_map(|cookie| {
                            let mut parts = cookie.trim().split('=');
                            if parts.next() == Some("Max-Age") {
                                Some(parts.next().unwrap_or("").trim())
                            } else {
                                None
                            }
                        })
                    });

                if max_age_str.is_none() {
                    return Err(Error::Auth);
                }

                let max_age: u64 = match max_age_str {
                    Some(age) => age.parse().unwrap_or(10),
                    _ => 10,
                };

                let mut meta = self.metadata.lock().await;
                meta.expires_at = SystemTime::now() + Duration::from_secs(max_age);
            } else {
                return Err(Error::Auth);
            }
        }

        Ok(())
    }

    pub async fn get_json<T>(&self, path: &str) -> Result<ApiResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        self.authenticate().await?;
        let res = self
            .client
            .get(format!("{}/api{}", self.server, path))
            .send()
            .await?;

        Ok(ApiResponse {
            status: res.status(),
            json: res.json().await?,
        })
    }

    pub async fn post<T>(&self, path: &str) -> Result<ApiResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        self.authenticate().await?;
        let res = self
            .client
            .post(format!("{}/api{}", self.server, path))
            .send()
            .await?;

        Ok(ApiResponse {
            status: res.status(),
            json: res.json().await?,
        })
    }

    pub async fn post_json<T, U>(&self, path: &str, body: &U) -> Result<ApiResponse<T>>
    where
        T: serde::de::DeserializeOwned,
        U: serde::ser::Serialize + ?Sized,
    {
        self.authenticate().await?;
        let res = self
            .client
            .post(format!("{}/api{}", self.server, path))
            .json(body)
            .send()
            .await?;

        Ok(ApiResponse {
            status: res.status(),
            json: res.json().await?,
        })
    }

    pub async fn delete<T>(&self, path: &str) -> Result<ApiResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        self.authenticate().await?;
        let res = self
            .client
            .delete(&format!("{}/api{}", self.server, path))
            .send()
            .await?;

        Ok(ApiResponse {
            status: res.status(),
            json: res.json().await?,
        })
    }
}
