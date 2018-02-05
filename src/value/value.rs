use std::iter::FromIterator;
use std::collections::HashMap;
use seckey::{SecKey, SecReadGuard};

/// Value is an enum that can contain any primitive value except for tuples
#[derive(Debug)]
pub enum Value {
  String(String),
  U64(u64),
  I64(i64),
  F64(f64),
  Bool(bool),
  HashMap(HashMap<String, Value>),
  Secret(SecKey<Value>),
  None,
}

unsafe impl Send for Value {}

pub type SecretGuard<'a> = SecReadGuard<'a, Value>;

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
  pub fn is_hash_map(&self) -> bool {
    self.as_hash_map().is_some()
  }
  pub fn is_none(&self) -> bool {
    match *self {
      Value::None => true,
      _ => false,
    }
  }
  pub fn is_secret(&self) -> bool {
    self.as_secret().is_some()
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
  pub fn as_hash_map(&self) -> Option<&HashMap<String, Value>> {
    match *self {
      Value::HashMap(ref v) => Some(v),
      _ => None,
    }
  }
  pub fn as_secret(&self) -> Option<SecretGuard> {
    match *self {
      Value::Secret(ref v) => Some(v.read()),
      _ => None,
    }
  }
}

impl<'a> From<&'a str> for Value {
  fn from(string: &'a str) -> Self {
    Value::from(string.to_owned())
  }
}

impl From<String> for Value {
  fn from(string: String) -> Self {
    // Secret Tag
    if string.starts_with("secret:") {
      let string = &string[7..];
      if string.starts_with("secret:") {
        panic!("Cannot repeat secret tag");
      }
      let value = Value::from(string);
      let sec_key = SecKey::new(value).unwrap();
      Value::from(sec_key)
    }
    // Map Tag
    else if string.starts_with("map:") {
      Value::from_src(&string[4..]).unwrap()
    }
    // Int Tag
    else if string.starts_with("int:") {
      let int = string[4..]
        .parse::<i64>()
        .expect(&format!("Invalid int tag {}", string));
      Value::I64(int)
    }
    // Float Tag
    else if string.starts_with("float:") {
      let float = string[6..]
        .parse::<f64>()
        .expect(&format!("Invalid float tag {}", string));
      Value::F64(float)
    }
    // Bool Tag
    else if string.starts_with("bool:") {
      if string == "bool:true" {
        Value::Bool(true)
      } else if string == "bool:false" {
        Value::Bool(false)
      } else {
        panic!("Invalid bool expression {}", string);
      }
    }
    // String
    else {
      Value::String(string)
    }
  }
}

impl From<u64> for Value {
  fn from(n: u64) -> Self {
    Value::U64(n)
  }
}

impl From<i64> for Value {
  fn from(n: i64) -> Self {
    Value::I64(n)
  }
}

impl From<f64> for Value {
  fn from(n: f64) -> Self {
    Value::F64(n)
  }
}

impl From<bool> for Value {
  fn from(boolean: bool) -> Self {
    Value::Bool(boolean)
  }
}

impl From<Vec<Value>> for Value {
  fn from(vec: Vec<Value>) -> Self {
    Value::HashMap(
      vec
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i.to_string(), v))
        .collect(),
    )
  }
}

impl From<HashMap<String, Value>> for Value {
  fn from(hash_map: HashMap<String, Value>) -> Self {
    Value::HashMap(hash_map)
  }
}

impl From<SecKey<Value>> for Value {
  fn from(sec_key: SecKey<Value>) -> Self {
    Value::Secret(sec_key)
  }
}

impl FromIterator<Value> for Value {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = Value>,
  {
    Value::HashMap(
      Vec::from_iter(iter)
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i.to_string(), v))
        .collect(),
    )
  }
}

impl FromIterator<(String, Value)> for Value {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = (String, Value)>,
  {
    Value::HashMap(HashMap::from_iter(iter))
  }
}
