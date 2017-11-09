use std::path::PathBuf;
use std::env::{current_dir, home_dir};
use super::Data;
use super::super::Error;

pub struct Inner<'a> {
  configuration_files: Vec<(String, Data<'a>)>,
  enviroment_variables: Data<'a>,
  command_line_flags: Data<'a>,
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

impl<'a> Inner<'a> {
  pub fn load(application_name: &str) -> Result<Self, Error> {
    Ok(Self {
      configuration_files: Self::gather_configuration_files(application_name)?,
      enviroment_variables: Self::gather_enviroment_variables(application_name)?,
      command_line_flags: Self::gather_command_line_flags(application_name)?,
    })
  }

  pub fn gather_configuration_files(application_name: &str) -> Result<Vec<(String, Data)>, Error> {
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

    let mut configuration_files = Vec::new();
    for path in paths {
      if !path.is_file() {
        continue;
      }

      let path_string = path.to_str().unwrap().to_string();
      let config_data = Data::from_path(&path).map_err(|_| Error::Noop)?;

      configuration_files.push((path_string, config_data));
    }

    Ok(configuration_files)
  }

  pub fn gather_enviroment_variables(application_name: &str) -> Result<Data, Error> {
    unimplemented!()
  }

  pub fn gather_command_line_flags(application_name: &str) -> Result<Data, Error> {
    unimplemented!()
  }
}
