use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PropertyType {
  Boolean,
  TimeOfDay,
  DayOfMonth,
  DayOfWeek,
  MonthOfYear,
  Timezone,
  Continuous,
  Periodic,
  Enum,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn property_type_from_json_string() {
    assert_eq!(
      PropertyType::TimeOfDay,
      serde_json::from_str("\"time_of_day\"").unwrap()
    );
    assert_eq!(
      PropertyType::DayOfMonth,
      serde_json::from_str("\"day_of_month\"").unwrap()
    );
    assert_eq!(
      PropertyType::DayOfWeek,
      serde_json::from_str("\"day_of_week\"").unwrap()
    );
    assert_eq!(
      PropertyType::MonthOfYear,
      serde_json::from_str("\"month_of_year\"").unwrap()
    );
    assert_eq!(
      PropertyType::Timezone,
      serde_json::from_str("\"timezone\"").unwrap()
    );
    assert_eq!(
      PropertyType::Continuous,
      serde_json::from_str("\"continuous\"").unwrap()
    );
    assert_eq!(
      PropertyType::Periodic,
      serde_json::from_str("\"periodic\"").unwrap()
    );
    assert_eq!(
      PropertyType::Enum,
      serde_json::from_str("\"enum\"").unwrap()
    );
  }
}
