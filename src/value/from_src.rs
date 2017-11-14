use std::fs::OpenOptions;
use std::path::Path;
use value::Value;
use std::io::{BufRead, BufReader, Cursor, Error as IoError, ErrorKind as IoErrorKind, Read, Seek,
              SeekFrom};
use error::Error;
use config::Kind;

impl Value {
  pub fn from_src(string: &str) -> Result<Self, Error> {
    Self::from_reader(Cursor::new(string))
  }

  pub fn from_path(path: &Path) -> Result<Self, Error> {
    Self::from_reader(BufReader::new(OpenOptions::new().read(true).open(path)?))
  }

  pub fn from_reader<R>(mut reader: R) -> Result<Self, Error>
  where
    R: Seek + BufRead,
  {
    let mut value = match Self::detect_config_type(&mut reader)? {
      Kind::Toml => Self::from_toml_reader(&mut reader)?,
      Kind::Json => Self::from_json_reader(&mut reader)?,
      Kind::Yaml => Self::from_yaml_reader(&mut reader)?,
    };
    value.eval(0);

    Ok(value)
  }

  fn detect_config_type<S>(reader: &mut S) -> Result<Kind, IoError>
  where
    S: BufRead + Seek,
  {
    reader.seek(SeekFrom::Start(0))?;

    let mut src_sample = String::new();
    for _ in 0..5 {
      if reader.read_line(&mut src_sample)? < 1 {
        break;
      };
    }
    src_sample.trim();

    let is_json = src_sample.starts_with("{");
    let is_yaml = false;
    let is_toml = src_sample.starts_with("[");

    if is_json {
      Ok(Kind::Json)
    } else if is_toml {
      Ok(Kind::Toml)
    } else if is_yaml {
      Ok(Kind::Yaml)
    } else {
      Err(IoError::new(IoErrorKind::Other, "Unsupported syntax"))
    }
  }

  fn from_toml_reader<R>(reader: &mut R) -> Result<Value, Error>
  where
    R: Read + Seek,
  {
    use toml::{from_str, Value as TomlValue};
    reader.seek(SeekFrom::Start(0))?;
    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();
    let toml_value: TomlValue = from_str(&string).map_err(|e| Error::from(e))?;
    Ok(toml_value.into())
  }

  fn from_json_reader<R>(reader: &mut R) -> Result<Value, Error>
  where
    R: Read + Seek,
  {
    use serde_json::{from_reader, Value as JsonValue};
    reader.seek(SeekFrom::Start(0))?;
    let json_value: JsonValue = from_reader(reader).map_err(|e| Error::from(e))?;
    Ok(json_value.into())
  }

  fn from_yaml_reader<R>(reader: &mut R) -> Result<Value, Error>
  where
    R: Read + Seek,
  {
    reader.seek(SeekFrom::Start(0))?;

    unimplemented!()
  }
}
