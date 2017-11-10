use serde_yaml::Value as YamlValue;
use value::Value;

impl<'a> From<YamlValue> for Value {
  fn from(yaml_value: YamlValue) -> Self {
    match yaml_value {
      YamlValue::Null => Value::None,
      YamlValue::Bool(b) => Value::Bool(b),
      YamlValue::Number(n) => {
        if let Some(n) = n.as_f64() {
          return Value::F64(n);
        }
        if let Some(n) = n.as_u64() {
          return Value::U64(n);
        }
        if let Some(n) = n.as_i64() {
          return Value::I64(n);
        }
        unreachable!()
      }
      YamlValue::String(s) => Value::String(s),
      YamlValue::Sequence(v) => Value::Vec(v.into_iter().map(|v| v.into()).collect()),
      YamlValue::Mapping(h) => Value::HashMap(
        h.into_iter()
          .map(|(key, value)| {
            let key = if let Some(s) = key.as_str() {
              s.to_owned()
            } else if let Some(i) = key.as_i64() {
              i.to_string()
            } else {
              unreachable!();
            };
            (key, value.into())
          })
          .collect(),
      ),
    }
  }
}
