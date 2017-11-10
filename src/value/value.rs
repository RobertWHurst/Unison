use std::collections::HashMap;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind as IoErrorKind, Seek, SeekFrom};
use error::Error;
use config::Kind;

/// Value is an enum that can contain any primitive value except for tuples
#[derive(Debug)]
pub enum Value {
  String(String),
  U64(u64),
  I64(i64),
  F64(f64),
  Bool(bool),
  Vec(Vec<Value>),
  HashMap(HashMap<String, Value>),
  None,
}

macro_rules! as_n {
  ($self:ident, $n:ty) => {
    match *$self {
      Value::U64(v) => if v < <$n>::max_value() as u64 {
        Some(v as $n)
      } else {
        None
      },
      Value::I64(v) => if v > <$n>::min_value() as i64 && v < <$n>::max_value() as i64 {
        Some(v as $n)
      } else {
        None
      },
      Value::F64(v) => if v > <$n>::min_value() as f64 && v < <$n>::max_value() as f64 {
        Some(v as $n)
      } else {
        None
      },
      _ => None,
    }
  }
}

impl Value {
  pub(crate) fn from_path(path: &Path) -> Result<Self, Error> {
    let mut reader = BufReader::new(OpenOptions::new().read(true).open(path)?);

    let mut value = match detect_config_type(&mut reader)? {
      Kind::Toml => parse_toml(&mut reader)?,
      Kind::Json => parse_json(&mut reader)?,
      Kind::Yaml => parse_yaml(&mut reader)?,
    };

    value.eval(0);

    Ok(value)
  }

  pub fn is_str(&self) -> bool {
    self.as_str().is_some()
  }
  pub fn is_u8(&self) -> bool {
    self.as_u8().is_some()
  }
  pub fn is_u16(&self) -> bool {
    self.as_u16().is_some()
  }
  pub fn is_u32(&self) -> bool {
    self.as_u32().is_some()
  }
  pub fn is_u64(&self) -> bool {
    self.as_u64().is_some()
  }
  pub fn is_i8(&self) -> bool {
    self.as_i8().is_some()
  }
  pub fn is_i16(&self) -> bool {
    self.as_i16().is_some()
  }
  pub fn is_i32(&self) -> bool {
    self.as_i32().is_some()
  }
  pub fn is_i64(&self) -> bool {
    self.as_i64().is_some()
  }
  pub fn is_f32(&self) -> bool {
    self.as_f32().is_some()
  }
  pub fn is_f64(&self) -> bool {
    self.as_f64().is_some()
  }
  pub fn is_bool(&self) -> bool {
    self.as_bool().is_some()
  }
  pub fn is_vec(&self) -> bool {
    self.as_vec().is_some()
  }
  pub fn is_hash_map(&self) -> bool {
    self.as_hash_map().is_some()
  }
  pub fn is_none(&self) -> bool {
    match *self {
      Value::None => true,
      _ => false,
    }
  }
  pub fn as_str(&self) -> Option<&str> {
    match *self {
      Value::String(ref v) => Some(v),
      _ => None,
    }
  }
  pub fn as_u8(&self) -> Option<u8> {
    as_n!(self, u8)
  }
  pub fn as_u16(&self) -> Option<u16> {
    as_n!(self, u16)
  }
  pub fn as_u32(&self) -> Option<u32> {
    as_n!(self, u32)
  }
  pub fn as_u64(&self) -> Option<u64> {
    as_n!(self, u64)
  }
  pub fn as_i8(&self) -> Option<i8> {
    as_n!(self, i8)
  }
  pub fn as_i16(&self) -> Option<i16> {
    as_n!(self, i16)
  }
  pub fn as_i32(&self) -> Option<i32> {
    as_n!(self, i32)
  }
  pub fn as_i64(&self) -> Option<i64> {
    as_n!(self, i64)
  }
  pub fn as_f32(&self) -> Option<f32> {
    match *self {
      Value::F64(v) => Some(v as f32),
      _ => None,
    }
  }
  pub fn as_f64(&self) -> Option<f64> {
    match *self {
      Value::F64(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_bool(&self) -> Option<bool> {
    match *self {
      Value::Bool(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_vec(&self) -> Option<&Vec<Value>> {
    match *self {
      Value::Vec(ref v) => Some(v),
      _ => None,
    }
  }
  pub fn as_hash_map(&self) -> Option<&HashMap<String, Value>> {
    match *self {
      Value::HashMap(ref v) => Some(v),
      _ => None,
    }
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

fn parse_toml(reader: &mut BufReader<File>) -> Result<Value, Error> {
  use toml::{from_str, Value as TomlValue};
  use std::io::Read;
  reader.seek(SeekFrom::Start(0))?;
  let mut string = String::new();
  reader.read_to_string(&mut string).unwrap();
  let toml_value: TomlValue = from_str(&string).map_err(|e| Error::from(e))?;
  Ok(toml_value.into())
}

fn parse_json(reader: &mut BufReader<File>) -> Result<Value, Error> {
  use serde_json::{from_reader, Value as JsonValue};
  reader.seek(SeekFrom::Start(0))?;
  let json_value: JsonValue = from_reader(reader).map_err(|e| Error::from(e))?;
  Ok(json_value.into())
}

fn parse_yaml(reader: &mut BufReader<File>) -> Result<Value, Error> {
  reader.seek(SeekFrom::Start(0))?;

  unimplemented!()
}
