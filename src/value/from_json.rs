use serde_json::Value as JsonValue;
use value::Value;

impl From<JsonValue> for Value {
  fn from(json_value: JsonValue) -> Self {
    match json_value {
      JsonValue::Null => Value::None,
      JsonValue::Bool(b) => Value::Bool(b),
      JsonValue::Number(n) => {
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
      JsonValue::String(s) => Value::String(s),
      JsonValue::Array(v) => Value::HashMap(
        v.into_iter()
          .enumerate()
          .map(|(i, v)| (i.to_string(), v.into()))
          .collect(),
      ),
      JsonValue::Object(h) => Value::HashMap(h.into_iter().map(|(k, v)| (k, v.into())).collect()),
    }
  }
}
