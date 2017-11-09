use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind as IoErrorKind, Seek, SeekFrom};
use std::collections::HashMap;
use super::super::Value;
use super::Kind;

pub struct Data<'a>(HashMap<String, Value<'a>>);

impl<'a> Data<'a> {
  pub fn from_path(path: &Path) -> Result<Self, IoError> {
    let mut reader = BufReader::new(OpenOptions::new().read(true).open(path)?);

    match detect_config_type(&mut reader)? {
      Kind::Toml => parse_toml(&mut reader)?,
      Kind::Json => parse_json(&mut reader)?,
      Kind::Yaml => parse_yaml(&mut reader)?,
    };

    Ok(Self { 0: HashMap::new() })
  }
}

fn detect_config_type(reader: &mut BufReader<File>) -> Result<Kind, IoError> {
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
  let is_toml = false;

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

fn parse_toml(reader: &mut BufReader<File>) -> Result<HashMap<String, Value>, IoError> {
  reader.seek(SeekFrom::Start(0))?;
  unimplemented!()
}

fn parse_json(reader: &mut BufReader<File>) -> Result<HashMap<String, Value>, IoError> {
  use serde_json::{from_reader, Map, Value as JsonValue};
  reader.seek(SeekFrom::Start(0))?;

  let root: JsonValue = from_reader(reader).map_err(|_| IoError::new(IoErrorKind::Other, ""))?;
  let root_obj = root
    .as_object()
    .ok_or(IoError::new(IoErrorKind::Other, ""))?;

  fn flatten_into_hash_map(
    cwp: &str,
    obj: &Map<String, JsonValue>,
    hash_map: &mut HashMap<String, Value>,
  ) {
    let cwp = cwp.to_string();
    for (key, value) in obj.iter() {}
  }

  let hash_map = HashMap::new();
  flatten_into_hash_map("", root_obj, &mut hash_map);

  println!("{:#?}", hash_map);
  unimplemented!()
}

fn parse_yaml(reader: &mut BufReader<File>) -> Result<HashMap<String, Value>, IoError> {
  reader.seek(SeekFrom::Start(0))?;

  unimplemented!()
}
