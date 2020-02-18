use crate::client::Client;
use crate::error::Error;

pub fn create_agent(_client: &Client, name: &str) -> Result<(), Error> {
  panic!("create_agent({}) - not implemented", name);
}

pub fn delete_agent(_client: &Client, name: &str) -> Result<(), Error> {
  panic!("delete_agent({}) - not implemented", name);
}
