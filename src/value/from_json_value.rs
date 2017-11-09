use serde_json::Value as JsonValue;
use super::Value;

impl<'a> From<JsonValue> for Value<'a> {
  fn from(json_value: JsonValue) -> Self {
    unimplemented!()
  }
}
