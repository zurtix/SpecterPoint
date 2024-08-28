use common::{
    error::{Error, Result},
    models::user::BaseCredential,
};
use reqwest::{cookie::Jar, StatusCode};
use sqlx::types::Json;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct ApiResponse<T = serde_json::Value> {
    pub status: StatusCode,
    pub json: Option<Json<T>>,
}

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

    pub async fn build(self) -> Result<Client> {
        let cookie_jar = Arc::new(Jar::default());
        let mut builder = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .cookie_provider(cookie_jar.clone());

        if let Some(p) = self.proxy {
            builder = builder.proxy(reqwest::Proxy::http(p).unwrap());
        }

        Client::new(self.creds, self.server, builder).await
    }
}

pub struct Client {
    server: String,
    creds: BaseCredential,
    client: reqwest::Client,
}

impl Client {
    async fn new(
        creds: BaseCredential,
        server: String,
        builder: reqwest::ClientBuilder,
    ) -> Result<Client> {
        let c = Self {
            server,
            creds,
            client: builder.build()?,
        };
        c.authenticate().await?;
        Ok(c)
    }

    async fn authenticate(&self) -> Result<()> {
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

    pub async fn get_json<ResponseType>(&self, path: &str) -> Result<ApiResponse<ResponseType>>
    where
        ResponseType: serde::de::DeserializeOwned,
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

    pub async fn post<ResponseType>(&self, path: &str) -> Result<ApiResponse<ResponseType>>
    where
        ResponseType: serde::de::DeserializeOwned,
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

    pub async fn post_json<ResponseType, BodyType>(
        &self,
        path: &str,
        body: &BodyType,
    ) -> Result<ApiResponse<ResponseType>>
    where
        ResponseType: serde::de::DeserializeOwned,
        BodyType: serde::ser::Serialize,
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

    pub async fn delete<ResponseType>(&self, path: &str) -> Result<ApiResponse<ResponseType>>
    where
        ResponseType: serde::de::DeserializeOwned,
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
