use craft_ai::ping;

mod common;

use crate::common::setup_client;

#[tokio::test]
async fn ping_works() {
  let client = setup_client();
  let ping_response = ping(&client).await.unwrap();
  assert!(ping_response.version != "");
}
