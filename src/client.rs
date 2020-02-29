use crate::error::Error;
use crate::token::TokenPayload;
use serde::{Deserialize, Serialize};
use std::fmt;

pub struct Client {
  pub token: String,
  pub url: String,
  pub owner: String,
  pub project: String,
  http_client: reqwest::Client,
}

#[derive(Deserialize)]
pub struct ApiError {
  pub message: String,
}

pub use reqwest::Method;

impl Client {
  pub fn from_token(token: &str) -> Result<Client, Error> {
    let token_payload = TokenPayload::from_token(token)?;
    let raw_token = token.to_string();

    Ok(Client {
      token: raw_token,
      url: token_payload.platform,
      owner: token_payload.owner,
      project: token_payload.project,
      http_client: reqwest::Client::new(),
    })
  }

  async fn request_url<
    ReqBodyT: Serialize,
    ResBodyT: for<'de> Deserialize<'de>,
    UrlT: Into<String>,
  >(
    &self,
    method: Method,
    url: UrlT,
    request_body: Option<&ReqBodyT>,
  ) -> Result<ResBodyT, Error> {
    let _url = url.into();
    let method_str = format!("{}", method);
    let mut request_builder = self.http_client.request(method, &_url).header(
      "Authorization",
      reqwest::header::HeaderValue::from_str(&format!("Bearer {}", self.token)).map_err(|err| {
        Error::BadToken(
          "Unable to create header value from the given token".to_string(),
          Some(Box::new(err)),
        )
      })?,
    );
    if let Some(body) = request_body {
      request_builder = request_builder.json(body);
    }
    let response = request_builder
      .send()
      .await
      .map_err(|err| Error::NetworkError(format!("Unable to reach '{}'", _url), Box::new(err)))?;
    let status_code = response.status();
    let handle_parse_error = |err| {
      Error::InternalError(
        format!(
          "Unable to parse response ({} '{}' -> {})",
          method_str, _url, status_code
        ),
        Some(Box::new(err)),
      )
    };
    if status_code.is_success() {
      Ok(
        response
          .json::<ResBodyT>()
          .await
          .map_err(handle_parse_error)?,
      )
    } else if status_code.is_client_error() {
      let api_error = response
        .json::<ApiError>()
        .await
        .map_err(handle_parse_error)?;

      Err(Error::InvalidArgument(format!(
        "{} ({} '{}' -> {})",
        api_error.message, method_str, _url, status_code
      )))
    } else {
      Err(Error::InternalError(
        format!(
          "Unexpected error ({} '{}' -> {})",
          method_str, _url, status_code
        ),
        None,
      ))
    }
  }

  pub async fn request_path<
    PathT: Into<String>,
    ReqBodyT: Serialize,
    ResBodyT: for<'de> Deserialize<'de>,
  >(
    &self,
    method: Method,
    path: PathT,
    request_body: Option<&ReqBodyT>,
  ) -> Result<ResBodyT, Error> {
    self
      .request_url(method, format!("{}{}", self.url, path.into()), request_body)
      .await
  }

  pub async fn request_project<
    PathT: Into<String>,
    ReqBodyT: Serialize,
    ResBodyT: for<'de> Deserialize<'de>,
  >(
    &self,
    method: Method,
    path: PathT,
    request_body: Option<&ReqBodyT>,
  ) -> Result<ResBodyT, Error> {
    self
      .request_url(
        method,
        format!(
          "{}/api/v1/{}/{}{}",
          self.url,
          self.owner,
          self.project,
          path.into()
        ),
        request_body,
      )
      .await
  }
}

impl fmt::Display for Client {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "project '{}/{}' @ '{}'",
      self.owner, self.project, self.url
    )
  }
}
