use std::env::args;
use std::collections::HashMap;
use config::ConfigError;
use value::Value;
use loaders::loader::Loader;
use schema::Schema;

/// Loads config values from command line flags. flags are mapped to config
/// paths by replacing `--` with `.`. Flags must start with `--`.
/// For example, `--server--port=8000` becomes `server.port` with a value of
/// `800` (parsed into the expected type provided by the schema). Note that
/// if a suffix is used then the suffix will be expected before the flag:
/// `suffix--server--port=8000`.
pub struct CliLoader {
  suffix: Option<String>,
}

impl CliLoader {
  /// Create a new Cli loader without a suffix.
  pub fn new() -> Self {
    Self { suffix: None }
  }

  /// Create a new Cli loader with a given suffix. The suffix will be expected
  /// at the start of all flags. It is recommended your suffix start with
  /// `--`.
  pub fn with_suffix(suffix: &str) -> Self {
    Self {
      suffix: Some(suffix.to_lowercase()),
    }
  }

  fn get_args(&self) -> Vec<String> {
    let mut args: Vec<_> = args().into_iter().skip(1).collect();
    if let Some(ref suffix) = self.suffix {
      args = args
        .into_iter()
        .filter(|arg| arg.to_lowercase().starts_with(suffix))
        .collect();
    }
    args
  }
}

impl Loader for CliLoader {
  fn collect(&mut self, _: &str, schema: &Schema) -> Result<Value, ConfigError> {
    let args = self.get_args();

    let flags: Vec<_> = args
      .into_iter()
      .filter(|a| a.starts_with("--"))
      .map(|arg| {
        let mut arg = arg[2..].split("=");

        let path = arg.next().unwrap().replace("--", ".").replace("-", "_");
        let raw_value = arg.next().map(|a| a.to_owned());

        (path, raw_value)
      })
      .filter(|&(ref p, _)| schema.has_path(&p))
      .collect();

    let mut values = Value::HashMap(HashMap::new());

    for (path, raw_value) in flags {
      let value = Value::from_raw_with_schema_and_path(raw_value, &path, schema)?;
      values.set_path(path, value);
    }

    Ok(values)
  }
}
