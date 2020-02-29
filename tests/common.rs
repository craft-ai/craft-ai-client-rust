use craft_ai::Client;
use dotenv::dotenv;
use std::env;
use std::sync::atomic::{AtomicU16, Ordering};

pub fn setup_client() -> Client {
  dotenv().ok();
  return Client::from_token(&env::var("CRAFT_TOKEN").unwrap()).unwrap();
}

static TEST_ENTITY_COUNTER: AtomicU16 = AtomicU16::new(0);

pub fn generate_entity_name() -> String {
  format!(
    "entity-{}",
    TEST_ENTITY_COUNTER.fetch_add(1, Ordering::Relaxed)
  )
}
