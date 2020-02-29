use crate::error::Error;
use crate::types::property::PropertyType;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyConfiguration {
  #[serde(rename = "type")]
  pub property_type: PropertyType,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_generated: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_optional: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentConfiguration {
  // Context configuration
  pub context: BTreeMap<String, PropertyConfiguration>,
  pub output: [String; 1],
  // Generation configuration
  #[serde(skip_serializing_if = "Option::is_none")]
  pub time_quantum: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub learning_period: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tree_max_depth: Option<usize>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub min_samples_per_leaf: Option<usize>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tree_max_operations: Option<usize>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub operations_as_events: Option<bool>,
  // Feature flags
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deactivate_covariance_split: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deactivate_forgetting: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deactivate_pruning: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub period_detection: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub forgetting_similarity_forgetting_ratio: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub forgetting_timestep: Option<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigurationBuilder {
  // Context configuration
  context: BTreeMap<String, PropertyConfiguration>,
  output: Option<String>,
  // Generation configuration
  time_quantum: Option<u64>,
  learning_period: Option<u64>,
  tree_max_depth: Option<usize>,
  min_samples_per_leaf: Option<usize>,
  tree_max_operations: Option<usize>,
  operations_as_events: Option<bool>,
  // Feature flags
  deactivate_covariance_split: Option<bool>,
  deactivate_forgetting: Option<bool>,
  deactivate_pruning: Option<bool>,
  period_detection: Option<bool>,
  forgetting_similarity_forgetting_ratio: Option<f64>,
  forgetting_timestep: Option<u64>,
}

impl ConfigurationBuilder {
  pub fn new() -> ConfigurationBuilder {
    ConfigurationBuilder {
      context: BTreeMap::new(),
      output: None,
      learning_period: None,
      time_quantum: None,
      tree_max_depth: None,
      min_samples_per_leaf: None,
      tree_max_operations: None,
      operations_as_events: None,
      deactivate_covariance_split: None,
      deactivate_forgetting: None,
      deactivate_pruning: None,
      period_detection: None,
      forgetting_similarity_forgetting_ratio: None,
      forgetting_timestep: None,
    }
  }

  pub fn add_property<'a, T: Into<String>>(
    &'a mut self,
    property_name: T,
    property_type: PropertyType,
    property_is_generated: Option<bool>,
    property_is_optional: Option<bool>,
  ) -> &'a mut ConfigurationBuilder {
    self.context.insert(
      property_name.into(),
      PropertyConfiguration {
        property_type: property_type,
        is_generated: property_is_generated,
        is_optional: property_is_optional,
      },
    );
    self
  }

  pub fn set_output_property<'a, T: Into<String>>(
    &'a mut self,
    property_name: T,
  ) -> &'a mut ConfigurationBuilder {
    self.output = Some(property_name.into());
    self
  }

  pub fn set_learning_period<'a>(
    &'a mut self,
    learning_period: u64,
  ) -> &'a mut ConfigurationBuilder {
    self.learning_period = Some(learning_period);
    self
  }

  pub fn create_agent_configuration(&self) -> Result<AgentConfiguration, Error> {
    Ok(AgentConfiguration {
      context: self.context.clone(),
      output: [self.output.clone().ok_or(Error::InvalidArgument(
        "No defined output property".to_string(),
      ))?],
      learning_period: self.learning_period,
      time_quantum: self.time_quantum,
      tree_max_depth: self.tree_max_depth,
      min_samples_per_leaf: self.min_samples_per_leaf,
      tree_max_operations: self.tree_max_operations,
      operations_as_events: self.operations_as_events,
      deactivate_covariance_split: self.deactivate_covariance_split,
      deactivate_forgetting: self.deactivate_forgetting,
      deactivate_pruning: self.deactivate_pruning,
      period_detection: self.period_detection,
      forgetting_similarity_forgetting_ratio: self.forgetting_similarity_forgetting_ratio,
      forgetting_timestep: self.forgetting_timestep,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn property_configuration_to_json_string() {
    assert_eq!(
      serde_json::to_string(&PropertyConfiguration {
        property_type: PropertyType::TimeOfDay,
        is_generated: Some(false),
        is_optional: None,
      })
      .unwrap(),
      "{\"type\":\"time_of_day\",\"is_generated\":false}"
    );
    assert_eq!(
      serde_json::to_string(&PropertyConfiguration {
        property_type: PropertyType::DayOfMonth,
        is_generated: None,
        is_optional: Some(true),
      })
      .unwrap(),
      "{\"type\":\"day_of_month\",\"is_optional\":true}"
    );
    assert_eq!(
      serde_json::to_string(&PropertyConfiguration {
        property_type: PropertyType::Enum,
        is_generated: None,
        is_optional: None,
      })
      .unwrap(),
      "{\"type\":\"enum\"}"
    );
  }
  #[test]
  fn property_configuration_from_json_string() {
    assert_eq!(
      serde_json::from_str::<PropertyConfiguration>(
        "{\"type\":\"month_of_year\",\"is_generated\":false}"
      )
      .unwrap(),
      PropertyConfiguration {
        property_type: PropertyType::MonthOfYear,
        is_generated: Some(false),
        is_optional: None,
      }
    );
    assert_eq!(
      serde_json::from_str::<PropertyConfiguration>(
        "{\"type\":\"continuous\",\"is_optional\":true}"
      )
      .unwrap(),
      PropertyConfiguration {
        property_type: PropertyType::Continuous,
        is_generated: None,
        is_optional: Some(true),
      }
    );
    assert_eq!(
      serde_json::from_str::<PropertyConfiguration>("{\"type\":\"periodic\"}").unwrap(),
      PropertyConfiguration {
        property_type: PropertyType::Periodic,
        is_generated: None,
        is_optional: None,
      }
    );
  }
  #[test]
  fn agent_configuration_no_output() {
    let error = ConfigurationBuilder::new()
      .add_property("x", PropertyType::Continuous, None, None)
      .add_property("y", PropertyType::Continuous, None, None)
      .create_agent_configuration()
      .unwrap_err();
    assert_eq!(
      error.to_string(),
      "Invalid argument - No defined output property"
    );
  }

  #[test]
  fn agent_configuration_to_json_string() {
    assert_eq!(
      serde_json::to_string(&ConfigurationBuilder::new()
      .add_property("time", PropertyType::TimeOfDay, Some(false), None)
      .add_property("day", PropertyType::DayOfWeek, None, None)
      .add_property("value", PropertyType::Continuous, None, None)
      .set_output_property("value")
      .create_agent_configuration()
      .unwrap())
      .unwrap(),
      "{\"context\":{\"day\":{\"type\":\"day_of_week\"},\"time\":{\"type\":\"time_of_day\",\"is_generated\":false},\"value\":{\"type\":\"continuous\"}},\"output\":[\"value\"]}"
    );
  }

  #[test]
  fn agent_configuration_from_json_string() {
    assert_eq!(
      serde_json::from_str::<AgentConfiguration>("{\"context\":{\"bar\":{\"type\":\"continuous\"},\"foo\":{\"type\":\"enum\",\"is_optional\":true},\"month\":{\"type\":\"month_of_year\"}},\"output\":[\"bar\"]}")
      .unwrap(),
      ConfigurationBuilder::new()
      .add_property("foo", PropertyType::Enum, None, Some(true))
      .add_property("bar", PropertyType::Continuous, None, None)
      .add_property("month", PropertyType::MonthOfYear, None, None)
      .set_output_property("bar")
      .create_agent_configuration()
      .unwrap()
    );
  }
}
