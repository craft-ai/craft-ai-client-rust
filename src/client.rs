use crate::error::Error;
use crate::token::TokenPayload;
use serde::Deserialize;
use std::fmt;

pub struct Client {
  pub token: String,
  pub url: String,
  pub owner: String,
  pub project: String,
  http_client: reqwest::Client,
}

impl Client {
  pub fn from_token(token: &str) -> Result<Client, Error> {
    let token_payload = TokenPayload::from_token(token)?;

    Ok(Client {
      token: token.to_string(),
      url: token_payload.platform,
      owner: token_payload.owner,
      project: token_payload.project,
      http_client: reqwest::Client::new(),
    })
  }

  pub async fn request<BodyT: for<'de> Deserialize<'de>>(
    &self,
    path: &str,
  ) -> Result<BodyT, Error> {
    let url = format!("{}/api/v1{}", self.url, path);
    let response = self
      .http_client
      .get(&url)
      .send()
      .await
      .map_err(|err| Error::NetworkError(format!("Unable to reach '{}'", url), Box::new(err)))?;
    let body = response.json::<BodyT>().await.map_err(|err| {
      Error::InternalError(
        format!("Unable to parse response from GET '{}'", url),
        Box::new(err),
      )
    })?;
    Ok(body)
  }
}

impl fmt::Display for Client {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "project {}/{} @ '{}'",
      self.owner, self.project, self.url
    )
  }
}
