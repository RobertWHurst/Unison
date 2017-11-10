use std::path::PathBuf;
use std::env::{current_dir, home_dir};
use value::Value;
use error::Error;

#[derive(Debug)]
pub struct Inner {
  pub(crate) config_files: Vec<(String, Value)>,
  pub(crate) env_vars: Value,
  pub(crate) cli_flags: Value,
}

// argv flags - Ex. --test--my-key val becomes config.test.myKey === 'val' in the config. Anything after -- is ignored.
// environment variables - Ex. APPLICATION_NAME__TEST__MY_KEY="val" becomes config.test.myKey === 'val'
// config files (replace {appname} with the name of your application) (Accepts JSON, INI, or YAML) (File extensions are optional)
// ~/.{appname}rc
// ~/.{appname}/config
// ~/.config/{appname}
// ~/.config/{appname}/config
// /etc/{appname}rc
// /etc/{appname}/config
// /usr/local/etc/{appname}rc
// /usr/local/etc/{appname}/config
// ./.{appname}rc
// ../.{appname}rc
// ../../.{appname}rc
// ../../../.{appname}rc
// ...

impl Inner {
  pub fn load(application_name: &str) -> Result<Self, Error> {
    Ok(Self {
      config_files: Self::gather_config_files(application_name)?,
      env_vars: Self::gather_env_vars(application_name)?,
      cli_flags: Self::gather_cli_flags(application_name)?,
    })
  }

  pub fn gather_config_files(application_name: &str) -> Result<Vec<(String, Value)>, Error> {
    let mut paths = Vec::new();

    if let Some(home_path) = home_dir() {
      paths.push(home_path.join(PathBuf::from(format!(".{}rc", application_name))));
      paths.push(home_path.join(PathBuf::from(format!(".{}/config", application_name))));
      paths.push(home_path.join(PathBuf::from(format!(".config/{}", application_name))));
      paths.push(home_path.join(PathBuf::from(
        format!(".config/{}/config", application_name),
      )));
    }

    paths.push(PathBuf::from(format!("/etc/{}rc", application_name)));
    paths.push(PathBuf::from(format!("/etc/{}/config", application_name)));
    paths.push(PathBuf::from(
      format!("/usr/local/etc/{}rc", application_name),
    ));
    paths.push(PathBuf::from(
      format!("/usr/local/etc/{}/config", application_name),
    ));

    let mut cwd_path = current_dir().unwrap();
    while let Some(_) = cwd_path.file_name() {
      paths.push(cwd_path.join(PathBuf::from(format!(".{}rc", application_name))));
      cwd_path.pop();
    }
    paths.push(PathBuf::from(format!("/.{}rc", application_name)));

    let mut config_files = Vec::new();
    for path in paths {
      if !path.is_file() {
        continue;
      }

      let path_string = path.to_str().unwrap().to_string();
      let config_data = Value::from_path(&path).map_err(|e| Error::from(e))?;

      config_files.push((path_string, config_data));
    }

    Ok(config_files)
  }

  pub fn gather_env_vars(application_name: &str) -> Result<Value, Error> {
    Ok(Value::None)
  }

  pub fn gather_cli_flags(application_name: &str) -> Result<Value, Error> {
    Ok(Value::None)
  }
}
