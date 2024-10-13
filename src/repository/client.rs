use log::error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Node {
    pub alias: String,
    pub capacity: u64,
    #[serde(rename = "firstSeen")]
    pub first_seen: i64,
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

pub struct Client {
    pub client: reqwest::Client,
}

impl Client {
    pub async fn new() -> Self {
        let client = reqwest::Client::new();

        return Client { client };
    }

    pub async fn get_nodes(&self) -> Result<Vec<Node>, reqwest::Error> {
        let response = self
            .client
            .get("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity")
            .header("Accept", "application/json")
            .send()
            .await;

        match response {
            Ok(resp) => match resp.json::<Vec<Node>>().await {
                Ok(data) => {
                    return Ok(data);
                }
                Err(e) => {
                    error!("Failed to parse response as JSON: {:?}", e);

                    return Ok(Vec::new());
                }
            },
            Err(e) => {
                error!("Failed to send request: {:?}", e);

                return Err(e);
            }
        };
    }
}
