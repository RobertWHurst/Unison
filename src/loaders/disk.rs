use config::ConfigError;
use value::Value;
use loaders::loader::Loader;
use schema::Schema;

pub struct DiskLoader {}

impl DiskLoader {
  pub fn new() -> Self {
    Self {}
  }
}

impl Loader for DiskLoader {
  fn collect(&mut self, _application_name: &str, _schema: &Schema) -> Result<Value, ConfigError> {
    unimplemented!()
  }
}

#[derive(Debug, Fail)]
pub enum DiskLoaderError {
  #[fail(display = "Unknown error")] Unknown,
}
