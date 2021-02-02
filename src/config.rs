use std::sync::{Arc, RwLock};
use crate::Value;

/// Config contains all loaded config data collected for the appication.
/// It can be cloned and passed accross threads as it useds Arc internally.
pub struct Config {
  inner: Arc<ConfigInner>
}

/// ConfigInner contains the data data for Config, and is meant to be wrapped
/// with an Arc.
struct ConfigInner {
  name: &'static str,
  value: RwLock<Value>
}

impl Config {

  /// new takes a string literal containing the name of your application. It
  /// returns a new Config instance.
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
