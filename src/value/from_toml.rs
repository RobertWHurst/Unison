use toml::Value as TomlValue;
use value::Value;

impl From<TomlValue> for Value {
  fn from(toml_value: TomlValue) -> Self {
    match toml_value {
      TomlValue::Boolean(b) => Value::Bool(b),
      TomlValue::Integer(n) => Value::I64(n),
      TomlValue::Float(f) => Value::F64(f),
      TomlValue::String(s) => Value::String(s),
      TomlValue::Datetime(d) => Value::String(d.to_string()),
      TomlValue::Array(v) => Value::HashMap(
        v.into_iter()
          .enumerate()
          .map(|(i, v)| (i.to_string(), v.into()))
          .collect(),
      ),
      TomlValue::Table(h) => Value::HashMap(h.into_iter().map(|(k, v)| (k, v.into())).collect()),
    }
  }
}
