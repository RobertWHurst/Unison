use std::sync::Arc;
use config::Inner;
use value::Value;
use error::Error;

#[derive(Debug, Clone)]
pub struct Config {
  application_name: String,
  inner: Arc<Inner>,
}

unsafe impl Send for Config {}

impl Config {
  pub fn load(application_name: &str) -> Result<Self, Error> {
    Ok(Self {
      application_name: application_name.to_string(),
      inner: Arc::new(Inner::load(application_name)?),
    })
  }

  pub fn reload(&mut self) -> Result<(), Error> {
    use std::mem;
    mem::replace(
      &mut self.inner,
      Arc::new(Inner::load(&self.application_name)?),
    );
    Ok(())
  }

  pub fn get(&self, path: &str) -> &Value {
    let flag_value = self.get_from_flag(path);
    if !flag_value.is_none() {
      return flag_value;
    }

    let env_value = self.get_from_env(path);
    if !env_value.is_none() {
      return env_value;
    }

    let config_value = self.get_from_config(path);
    if !config_value.is_none() {
      return config_value;
    }

    &Value::None
  }

  pub fn get_from_config(&self, path: &str) -> &Value {
    for &(_, ref value) in self.inner.config_files.iter() {
      let value = value.get(path);
      if !value.is_none() {
        return value;
      }
    }
    &Value::None
  }

  pub fn get_from_config_at_path(&self, fs_path: &str, path: &str) -> &Value {
    let config_files = &self.inner.config_files;
    let value = match config_files.iter().find(|&&(ref p, _)| p == fs_path) {
      Some(c) => c,
      None => return &Value::None,
    };
    value.1.get(path)
  }

  pub fn get_from_flag(&self, path: &str) -> &Value {
    self.inner.cli_flags.get(path)
  }

  pub fn get_from_env(&self, path: &str) -> &Value {
    self.inner.env_vars.get(path)
  }
}
