use config::ConfigError;
use value::Value;
use loaders::loader::Loader;
use schema::Schema;

pub struct EnvLoader {}

impl EnvLoader {
  pub fn new() -> Self {
    Self {}
  }
}

impl Loader for EnvLoader {
  fn collect(&mut self, _application_name: &str, _schema: &Schema) -> Result<Value, ConfigError> {
    unimplemented!()
  }
}

#[derive(Debug, Fail)]
pub enum EnvLoaderError {
  #[fail(display = "Unknown error")] Unknown,
}
