use std::sync::{Arc, RwLock};
use crate::Value;

pub struct Config {
  inner: Arc<ConfigInner>
}

struct ConfigInner {
  name: &'static str,
  value: RwLock<Value>
}

impl Config {
  pub fn new(name: &'static str) -> Self {
    Self {
      inner: Arc::new(ConfigInner {
        name,
        value: RwLock::new(Value::None)
      })
    }
  }
}

#[test]
fn can_create_config() {
  let config = Config::new("test");
}
