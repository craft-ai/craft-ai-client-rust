use crate::client::{Client, Method};
use crate::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiInfo {
  #[serde(rename(deserialize = "craft_version"))]
  pub version: String,
  pub activated_features: Vec<String>,
}

pub async fn ping(client: &Client) -> Result<ApiInfo, Error> {
  client
    .request_path::<&str, (), ApiInfo>(Method::GET, "/api/v1", None)
    .await
}
