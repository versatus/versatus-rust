use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApiResponse {
    // Define response structure
}

pub struct NetworkClient {
    client: Client,
    base_url: String,
}

impl NetworkClient {
    pub fn new(base_url: String) -> Self {
        NetworkClient {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get_data(&self, endpoint: &str) -> Result<ApiResponse> {
        let url = format!("{}/{}", self.base_url, endpoint);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<ApiResponse>()
            .await
            .map_err(|e| anyhow::anyhow!("{e:?}"))
    }

    pub async fn send_command(&self, endpoint: &str, instruction: String) -> Result<ApiResponse> {
        let url = format!("{}/{}", self.base_url, endpoint);
        self.client
            .put(&url)
            .send()
            .await?
            .json::<ApiResponse>()
            .await
            .map_err(|e| anyhow::anyhow!("{e:?}"))
    }
}
