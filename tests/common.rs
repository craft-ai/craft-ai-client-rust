use dotenv::dotenv;
use std::env;

use craft_ai::Client;

pub fn setup_client() -> Client {
  dotenv().ok();
  return Client::from_token(&env::var("CRAFT_TOKEN").unwrap()).unwrap();
}
