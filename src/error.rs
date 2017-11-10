use std::io::Error as IoError;
use toml::de::Error as SerdeTomlError;
use serde_json::Error as SerdeJsonError;
use serde_yaml::Error as SerdeYamlError;

#[derive(Debug)]
pub enum Error {
  Io(IoError),
  Toml(SerdeTomlError),
  Json(SerdeJsonError),
  Yaml(SerdeYamlError),
}

impl From<IoError> for Error {
  fn from(e: IoError) -> Self {
    Error::Io(e)
  }
}

impl From<SerdeTomlError> for Error {
  fn from(e: SerdeTomlError) -> Self {
    Error::Toml(e)
  }
}

impl From<SerdeJsonError> for Error {
  fn from(e: SerdeJsonError) -> Self {
    Error::Json(e)
  }
}

impl From<SerdeYamlError> for Error {
  fn from(e: SerdeYamlError) -> Self {
    Error::Yaml(e)
  }
}
