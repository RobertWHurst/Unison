use std::collections::hash_map;
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
    if let Some(hash_map) = self.as_hash_map() {
      return hash_map.get(key).unwrap_or(&Value::None);
    };
    if let Some(vec) = self.as_vec() {
      let i = match key.parse::<usize>() {
        Ok(i) => i,
        Err(_) => return &Value::None,
      };
      return vec.get(i).unwrap_or(&Value::None);
    };
    &Value::None
  }
}
