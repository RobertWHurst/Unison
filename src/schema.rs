use std::any::Any;
use std::collections::HashMap;

/// Schema sets the structure and and default values of your configuration
/// sources. A schema must be given when creating instances of `unison::Config`.
#[derive(Debug)]
pub struct Schema {
  paths: HashMap<String, Box<Any + Send>>,
}

impl Schema {
  /// Creates a new schema builder which can be used to configure and build a
  /// new schema.
  pub fn new() -> SchemaBuilder {
    SchemaBuilder {
      paths: HashMap::new(),
    }
  }

  /// Returns a boolean indicating if the schema contains a given path.
  pub fn has_path(&self, path_name: &str) -> bool {
    self.paths.contains_key(path_name)
  }

  /// Returns a boolean indicating if the given schema path is the same type
  /// as the given type argument `C`.
  pub fn path_is_type<C: Any + Send>(&self, path_name: &str) -> bool {
    match self.paths.get(path_name) {
      Some(p) => p.is::<C>(),
      None => false,
    }
  }

  /// Returns the option wrapped default value at the given path if present.
  pub fn path_default<C: Any + Send>(&self, path_name: &str) -> Option<&C> {
    self.paths.get(path_name)?.downcast_ref::<C>()
  }
}

/// Used to configure and build `unision::Schema` instances.
pub struct SchemaBuilder {
  paths: HashMap<String, Box<Any + Send>>,
}

impl SchemaBuilder {
  /// Adds a path with a default value to the schema to be built.
  pub fn path<K: Any + Send>(mut self, path_name: &str, default_value: K) -> Self {
    self.paths.insert(path_name.into(), Box::new(default_value));
    self
  }

  /// Builds and returns a schema from the paths set on the builder.
  pub fn build(self) -> Schema {
    Schema { paths: self.paths }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_create_schema() {
    let _: Schema = Schema::new()
      .path("server.port", 3000)
      .path("server.url", "http://test.url")
      .build();
  }

  #[test]
  fn can_get_schema_path_default_value() {
    let schema = Schema::new().path::<u32>("server.port", 3000).build();

    let default_value = schema.path_default::<u32>("server.port").unwrap();

    assert_eq!(*default_value, 3000);
  }

  #[test]
  fn can_check_schema_path_type() {
    let schema = Schema::new().path::<u32>("server.port", 3000).build();

    assert!(schema.path_is_type::<u32>("server.port"));
  }

  #[test]
  fn can_send_schema_across_threads() {
    use std::thread;

    let schema = Schema::new().path::<u32>("server.port", 3000).build();

    thread::spawn(move || {
      let _: u32 = *schema.path_default::<u32>("server.port").unwrap();
    });
  }
}
