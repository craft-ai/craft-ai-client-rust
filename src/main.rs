use clap::{load_yaml, App};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

use dotenv::dotenv;

use craft_ai::{
  create_agent, create_agent_with_generated_name, delete_agent, ping, AgentConfiguration, Client,
  DeleteAgentResponse, Error,
};

fn deserialize_json_file<ContentT: for<'de> Deserialize<'de>, PathT: Into<String>>(
  path: PathT,
) -> Result<ContentT, Error> {
  let path_str = path.into();
  let file = File::open(&path_str)
    .map_err(|_err| Error::InvalidArgument(format!("Unable to open '{}'.", path_str)))?;
  let mut buf_reader = BufReader::new(file);
  let mut file_content = String::new();
  buf_reader
    .read_to_string(&mut file_content)
    .map_err(|_err| Error::InvalidArgument(format!("Unable to read data from '{}'.", path_str)))?;
  Ok(
    serde_json::from_str::<ContentT>(&file_content).map_err(|_err| {
      Error::InvalidArgument(format!("Unable to deserialize from '{}'.", path_str))
    })?,
  )
}

#[tokio::main]
async fn main() {
  // Load dotenv
  dotenv().ok();

  let yaml = load_yaml!("cli.yml");
  let app = App::from_yaml(yaml);
  let matches = app.get_matches();

  let token_from_env = env::var("CRAFT_TOKEN").unwrap_or("".to_string());
  let token = matches.value_of("token").unwrap_or(&token_from_env);
  let client = Client::from_token(token).unwrap();
  println!("Interacting with {}.", client);

  if let Some(_) = matches.subcommand_matches("ping") {
    let response = ping(&client).await.unwrap();
    println!(
      "Remote craft ai uses version {} with the following activated features {:?}",
      response.version, response.activated_features
    );
    std::process::exit(0);
  }
  if let Some(agent_matches) = matches.subcommand_matches("agents") {
    if let Some(create_agent_matches) = agent_matches.subcommand_matches("create") {
      let configuration_file_path = create_agent_matches
        .value_of("AGENT_CONFIGURATION_FILE")
        .unwrap();
      let configuration =
        deserialize_json_file::<AgentConfiguration, &str>(configuration_file_path).unwrap();
      let created_agent = match create_agent_matches.value_of("name") {
        Some(agent_name) => create_agent(&client, agent_name, &configuration)
          .await
          .unwrap(),
        None => create_agent_with_generated_name(&client, &configuration)
          .await
          .unwrap(),
      };
      println!(
        "Sucessfully created agent '{}/{}/{}'\n---\n{}",
        client.owner,
        client.project,
        created_agent.name,
        serde_json::to_string_pretty(&created_agent).unwrap()
      );
      std::process::exit(0);
    }
    if let Some(create_agent_matches) = agent_matches.subcommand_matches("delete") {
      let name = create_agent_matches.value_of("NAME").unwrap();
      match delete_agent(&client, name).await.unwrap() {
        DeleteAgentResponse::Deleted(deleted_agent_name) => {
          println!(
            "Sucessfully deleted agent '{}/{}/{}'",
            client.owner, client.project, deleted_agent_name
          );
        }
        DeleteAgentResponse::NonExisting(deleted_agent_name) => {
          println!(
            "No agent named '{}/{}/{}'",
            client.owner, client.project, deleted_agent_name
          );
        }
      }
      std::process::exit(0);
    }
  }

  panic!("Unhandled CLI options");
}
