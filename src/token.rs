use crate::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TokenPayload {
  pub platform: String,
  pub owner: String,
  pub project: String,
}

impl TokenPayload {
  pub fn from_token(token: &str) -> Result<TokenPayload, Error> {
    if token.len() == 0 {
      return Err(Error::BadToken("Empty token".to_string(), None));
    }
    let raw_segments: Vec<&str> = token.split(".").collect();
    let segments_count = raw_segments.len();
    if segments_count != 3 {
      return Err(Error::BadToken(
        format!(
          "Invalid number of segment in given JWT token: 3 segments expected, {} found",
          segments_count
        ),
        None,
      ));
    }

    let raw_payload_segment = raw_segments[1];
    let decoded_payload_segment = base64::decode(raw_payload_segment).map_err(|err| {
      Error::BadToken(
        "Unable to decode JWT token segment from base64".to_string(),
        Some(Box::new(err)),
      )
    })?;

    let payload: TokenPayload = serde_json::from_slice(decoded_payload_segment.as_slice())
      .map_err(|err| {
        Error::BadToken(
          "Unable to deserialize JWT token segment from json".to_string(),
          Some(Box::new(err)),
        )
      })?;

    Ok(payload)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_token_empty() {
    let error = TokenPayload::from_token("").unwrap_err();
    assert_eq!(error.to_string(), "Bad token - Empty token");
  }

  #[test]
  fn from_token_one_segment() {
    let error = TokenPayload::from_token("foo").unwrap_err();
    assert_eq!(
      error.to_string(),
      "Bad token - Invalid number of segment in given JWT token: 3 segments expected, 1 found"
    );
  }

  #[test]
  fn from_token_two_segments() {
    let error = TokenPayload::from_token("foo.bar").unwrap_err();
    assert_eq!(
      error.to_string(),
      "Bad token - Invalid number of segment in given JWT token: 3 segments expected, 2 found"
    );
  }

  #[test]
  fn from_token_bad_base64() {
    let error = TokenPayload::from_token("foobarbaz.foobarbaz.foobarbaz").unwrap_err();
    assert_eq!(
      error.to_string(),
      "Bad token - Unable to decode JWT token segment from base64"
    );
  }

  #[test]
  fn from_token_bad_json() {
    let error =
      TokenPayload::from_token("Zm9vYmFyYmF6Cg==.Zm9vYmFyYmF6Cg==.Zm9vYmFyYmF6Cg==").unwrap_err();
    assert_eq!(
      error.to_string(),
      "Bad token - Unable to deserialize JWT token segment from json"
    );
  }

  #[test]
  fn from_token_bad_json_format() {
    let error =
      TokenPayload::from_token("eyJmb28iOjB9Cg==.eyJmb28iOjB9Cg==.eyJmb28iOjB9Cg==").unwrap_err();
    assert_eq!(
      error.to_string(),
      "Bad token - Unable to deserialize JWT token segment from json"
    );
  }

  #[test]
  fn from_token_valid() {
    let token_payload = TokenPayload::from_token("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJvd25lciI6ImNsb2RlcmljIiwicHJvamVjdCI6InNhbmRib3giLCJyaWdodCI6IndyaXRlIiwicGxhdGZvcm0iOiJodHRwczovL2JldGEuY3JhZnQuYWkiLCJpYXQiOjE0ODkxNDE3MDcsImlzcyI6Imh0dHBzOi8vYmV0YS5jcmFmdC5haSIsImp0aSI6ImU1NWQyMzYyLTZlOTAtNDlhMy05ZGQwLTM5ZTI4Mjc3ZTNkNiJ9.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
    assert_eq!(token_payload.platform, "https://beta.craft.ai");
    assert_eq!(token_payload.owner, "cloderic");
    assert_eq!(token_payload.project, "sandbox");
  }
}
