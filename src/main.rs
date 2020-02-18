use std::env;

use clap::{load_yaml, App};

use dotenv::dotenv;

use craft_ai::{create_agent, delete_agent, ping, Client};

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
      let name = create_agent_matches.value_of("NAME").unwrap();
      create_agent(&client, name).unwrap();
      std::process::exit(0);
    }
    if let Some(create_agent_matches) = agent_matches.subcommand_matches("delete") {
      let name = create_agent_matches.value_of("NAME").unwrap();
      delete_agent(&client, name).unwrap();
      std::process::exit(0);
    }
  }

  panic!("Unhandled CLI options");
}
