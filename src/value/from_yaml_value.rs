use serde_yaml::Value as YamlValue;
use super::Value;

impl<'a> From<YamlValue> for Value<'a> {
  fn from(json_value: YamlValue) -> Self {
    unimplemented!()
  }
}
