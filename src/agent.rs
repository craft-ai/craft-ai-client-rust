use crate::client::{ApiError, Client, Method};
use crate::error::Error;
use crate::types::AgentConfiguration;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct CreateAgentReqBody<'a> {
  #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  pub configuration: &'a AgentConfiguration,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Agent {
  #[serde(rename = "id")]
  pub name: String,
  pub configuration: AgentConfiguration,
}

pub async fn create_agent<T: Into<String>>(
  client: &Client,
  name: T,
  configuration: &AgentConfiguration,
) -> Result<Agent, Error> {
  let req_body = CreateAgentReqBody {
    name: Some(name.into()),
    configuration: configuration,
  };
  client
    .request_project::<&str, CreateAgentReqBody, Agent>(Method::POST, "/agents", Some(&req_body))
    .await
}

pub async fn create_agent_with_generated_name(
  client: &Client,
  configuration: &AgentConfiguration,
) -> Result<Agent, Error> {
  let req_body = CreateAgentReqBody {
    name: None,
    configuration: configuration,
  };
  client
    .request_project::<&str, CreateAgentReqBody, Agent>(Method::POST, "/agents", Some(&req_body))
    .await
}

#[derive(Deserialize)]
#[serde(untagged)]
enum DeleteAgentResBody {
  Deleted(Agent),
  NonExisting(ApiError),
}

#[derive(Debug, PartialEq)]
pub enum DeleteAgentResponse {
  Deleted(String),
  NonExisting(String),
}

pub async fn delete_agent<T: Into<String>>(
  client: &Client,
  name: T,
) -> Result<DeleteAgentResponse, Error> {
  let name_str = name.into();
  match client
    .request_project::<String, (), DeleteAgentResBody>(
      Method::DELETE,
      format!("/agents/{}", name_str),
      None,
    )
    .await?
  {
    DeleteAgentResBody::Deleted(agent) => Ok(DeleteAgentResponse::Deleted(agent.name)),
    DeleteAgentResBody::NonExisting(_) => Ok(DeleteAgentResponse::NonExisting(name_str)),
  }
}
