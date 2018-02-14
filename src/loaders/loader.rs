use schema::Schema;
use value::Value;
use config::ConfigError;

pub trait Loader {
  fn collect(&mut self, application_name: &str, schema: &Schema) -> Result<Value, ConfigError>;
}
