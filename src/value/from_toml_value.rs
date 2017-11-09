use toml::Value as TomlValue;
use super::Value;

impl<'a> From<TomlValue> for Value<'a> {
  fn from(json_value: TomlValue) -> Self {
    unimplemented!()
  }
}
