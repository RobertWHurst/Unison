use std::collections::HashMap;
use failure::Error;
use schema::Schema;
// use loaders::{CliLoader, DiskLoader, EnvLoader, Loader};
use loaders::{CliLoader, Loader};
use value::{FromValue, IntoValue, Value};

/// Config collects and contains configuration information.
/// This is the struct you should be interacting with the most from the unison
/// crate.
#[derive(Debug)]
pub struct Config {
  application_name: String,
  schema: Schema,
  values: Vec<Value>,
}

impl Config {
  /// Create a new config and executes all loaders at once.
  /// This is equivelent to the following:
  ///
  /// ```rust
  /// let config = Config::new();
  /// config.init();
  /// ```
  pub fn load(application_name: &str, schema: Schema) -> Result<Self, ConfigError> {
    let mut config = Self::new(application_name, schema);
    config.init()?;
    Ok(config)
  }

  /// Creates a new config.
  /// The provided application name will be used by loaders to find
  /// configuration sources intented for your application on the system.
  /// The schema will be used to validate and filter config values.
  pub fn new(application_name: &str, schema: Schema) -> Self {
    let application_name = application_name.to_owned();
    Self {
      application_name,
      schema,
      values: Vec::new(),
    }
  }

  /// Executes all loaders and collects all config values.
  pub fn init(&mut self) -> Result<(), ConfigError> {
    // NOTE: This could be passed in future versons of unison to allow for expantion of the system
    let loaders: Vec<Box<Loader>> = vec![
      // Box::new(DiskLoader::new()),
      // Box::new(EnvLoader::new()),
      Box::new(CliLoader::new()),
    ];

    for mut loader in loaders {
      self
        .values
        .push(loader.collect(&self.application_name, &self.schema)?);
    }
    // NOTE: This final value is inserted in order to contain any overrides set
    // using `Config::set`.
    self.values.push(Value::HashMap(HashMap::new()));

    Ok(())
  }

  /// Gets a value of a given path. If the path exists then the option wrapped
  /// value will be returned. If no path is found then a none will be returned
  /// instead.
  pub fn try_get<P: AsRef<str>, T: FromValue>(&self, path: P) -> Option<T> {
    let path = path.as_ref();
    for value in self.values.iter() {
      match *value.get_path(path) {
        Value::None => continue,
        ref v => {
          return match v.to_owned().into_type::<T>() {
            Ok(v) => Some(v),
            Err(_) => None,
          }
        }
      };
    }
    None
  }

  /// Gets a value of a given path. Panics if the path does not exist.
  pub fn get<P: AsRef<str>, T: FromValue>(&self, path: P) -> T {
    self.try_get(path).unwrap()
  }

  /// Override a config value at a given path with a given value. Note that
  /// this value will not be presisted to configuration sources.
  pub fn set<P: AsRef<str>, T: IntoValue>(&mut self, path: P, value: T) -> Result<(), ConfigError> {
    let value = value.into_value();
    let target = self.values.last_mut().ok_or(ConfigError::NotInitialized)?;
    target.set_path(path, value);
    Ok(())
  }
}

/// An error type that can be returned from any of the error producing `Config`
/// methods.
#[derive(Debug, Fail)]
pub enum ConfigError {
  #[fail(display = "Loader Error: {}", _0)] LoaderError(#[fail(cause)] Error),
  #[fail(display = "Not yet initialized. You must call `Config::init` first")] NotInitialized,
  #[fail(display = "Cannot set value. Missing path")] MissingSetPath,
  #[fail(display = "Cannot parse value at path {} into type {}", _0, _1)]
  BadPathType(String, &'static str),
  #[fail(display = "Unknown error")] Unknown,
}
