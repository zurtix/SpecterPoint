pub struct ClientBuilder {
    username: String,
    password: String,
    server: String,
    proxy: Option<String>,
}

impl ClientBuilder {
    pub fn new(self, username: String, password: String, server: String) -> Self {
        Self {
            username,
            password,
            server,
            proxy: None,
        }
    }

    pub fn proxy(&mut self, proxy: String) {
        self.proxy = Some(proxy)
    }

    pub fn build(self) -> Client {
        let mut http = reqwest::ClientBuilder::new();

        if let Some(p) = self.proxy {
            http = http.proxy(reqwest::Proxy::http(&p).unwrap());
        }

        Client {
            username: self.username,
            password: self.password,
            server: self.server,
            http: http.build().unwrap(),
        }
    }
}

pub struct Client {
    username: String,
    password: String,
    server: String,
    http: reqwest::Client,
}

impl Client {
    async fn authenticate(self) {
        let res = self
            .http
            .post(format!("{}/api/login", self.server))
            .basic_auth(self.username, Some(self.password))
            .send()
            .await
            .unwrap();
    }
}
