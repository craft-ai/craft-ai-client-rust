use craft_ai::{
  create_agent, create_agent_with_generated_name, delete_agent, ConfigurationBuilder,
  DeleteAgentResponse, PropertyType,
};

mod common;

use crate::common::{generate_entity_name, setup_client};

#[tokio::test]
async fn create_and_delete_agent() {
  let client = setup_client();
  let agent_name = generate_entity_name();

  // Make sur the agent is deleted first
  delete_agent(&client, &agent_name).await.unwrap();

  // create a configuration
  let configuration = ConfigurationBuilder::new()
    .add_property("time", PropertyType::TimeOfDay, None, None)
    .add_property("tz", PropertyType::Timezone, None, None)
    .add_property("value", PropertyType::Continuous, None, None)
    .set_output_property("value")
    .create_agent_configuration()
    .unwrap();

  // create the agent
  let created_agent = create_agent(&client, &agent_name, &configuration)
    .await
    .unwrap();
  assert_eq!(created_agent.name, agent_name);
  assert!(created_agent
    .configuration
    .context
    .keys()
    .eq(configuration.context.keys()));
  assert_eq!(created_agent.configuration.output, configuration.output);

  // delete the agent
  assert_eq!(
    delete_agent(&client, &agent_name).await.unwrap(),
    DeleteAgentResponse::Deleted(agent_name)
  )
}

#[tokio::test]
async fn create_and_delete_agent_with_generated_name() {
  let client = setup_client();

  // create a configuration
  let configuration = ConfigurationBuilder::new()
    .add_property("x", PropertyType::Enum, None, None)
    .add_property("y", PropertyType::Continuous, None, Some(true))
    .add_property("value", PropertyType::Boolean, None, None)
    .set_output_property("value")
    .create_agent_configuration()
    .unwrap();

  // create the agent
  let created_agent = create_agent_with_generated_name(&client, &configuration)
    .await
    .unwrap();
  assert!(created_agent
    .configuration
    .context
    .keys()
    .eq(configuration.context.keys()));
  assert_eq!(created_agent.configuration.output, configuration.output);

  // delete the agent
  assert_eq!(
    delete_agent(&client, &created_agent.name).await.unwrap(),
    DeleteAgentResponse::Deleted(created_agent.name)
  )
}

#[tokio::test]
async fn delete_non_existing_agent() {
  let client = setup_client();
  let nonexisting_agent_name = "this-agent-does-not-exist";

  // delete the agent
  assert_eq!(
    delete_agent(&client, nonexisting_agent_name).await.unwrap(),
    DeleteAgentResponse::NonExisting(nonexisting_agent_name.to_string())
  )
}
