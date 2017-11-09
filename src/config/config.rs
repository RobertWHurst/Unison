use std::sync::Arc;

use super::Inner;
use super::super::Value;
use super::super::Error;

pub struct Config<'a> {
  inner: Arc<Inner<'a>>,
}

impl<'a> Config<'a> {
  pub fn load(application_name: &str) -> Result<Self, Error> {
    Ok(Self {
      inner: Arc::new(Inner::load(application_name)?),
    })
  }

  pub fn get(&self, path: &str) -> Option<Value> {
    unimplemented!()
  }

  pub fn get_from_config(&self, path: &str) -> Option<Value> {
    unimplemented!()
  }

  pub fn get_from_flag(&self, path: &str) -> Option<Value> {
    unimplemented!()
  }

  pub fn get_from_env(&self, path: &str) -> Option<Value> {
    unimplemented!()
  }
}
