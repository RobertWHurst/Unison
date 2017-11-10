use std::collections::HashMap;
use value::Value;

impl Value {
  pub fn get(&self, path: &str) -> &Value {
    let path_chunks = path.split(".");
    let mut ctx = self;
    for chunk in path_chunks {
      ctx = ctx.get_key(chunk);
    }
    ctx
  }

  fn get_key(&self, key: &str) -> &Value {
    match *self {
      Value::HashMap(ref h) => h.get(key).unwrap_or(&Value::None),
      _ => &Value::None,
    }
  }

  fn get_key_mut(&mut self, key: &str) -> Option<&mut Value> {
    match *self {
      Value::HashMap(ref mut h) => h.get_mut(key),
      _ => None,
    }
  }

  pub fn set(&mut self, path: &str, mut value: Value) {
    let mut chunks: Vec<_> = path.split(".").collect();
    if chunks.len() == 0 {
      return;
    }
    let key = chunks.remove(0);

    if chunks.len() != 0 {
      if let Some(ref mut next_value) = self.get_key_mut(key) {
        return next_value.set(&chunks.join("."), value);
      }
    }

    chunks.reverse();
    while let Some(chunk) = chunks.pop() {
      value = {
        match chunk.parse::<usize>() {
          Ok(i) => {
            let mut vec = Vec::new();
            vec.insert(i, value);
            Value::from(vec)
          }
          Err(_) => {
            let mut hash_map = HashMap::new();
            hash_map.insert(chunk.to_owned(), value);
            Value::from(hash_map)
          }
        }
      };
    }

    self.set_key(key, value);
  }

  fn set_key(&mut self, key: &str, value: Value) {
    match *self {
      Value::HashMap(ref mut h) => {
        h.insert(key.to_string(), value);
      }
      _ => (),
    }
  }
}
