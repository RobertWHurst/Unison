use std::collections::HashMap;
use std::ops::Index;

/// Value is an enum that can contain any primitive value except for tuples
pub enum Value<'a> {
  String(String),
  U8(u8),
  U16(u16),
  U32(u32),
  U64(u64),
  I8(i8),
  I16(i16),
  I32(i32),
  I64(i64),
  F32(f32),
  F64(f64),
  Bool(bool),
  Vec(Vec<Value<'a>>),
  HashMap(HashMap<String, Value<'a>>),
}

impl<'a> Value<'a> {
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
  pub fn as_str(&self) -> Option<&'a str> {
    match self {
      &Value::String(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_u8(&self) -> Option<u8> {
    match self {
      &Value::U8(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_u16(&self) -> Option<u16> {
    match self {
      &Value::U16(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_u32(&self) -> Option<u32> {
    match self {
      &Value::U32(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_u64(&self) -> Option<u64> {
    match self {
      &Value::U64(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_i8(&self) -> Option<i8> {
    match self {
      &Value::I8(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_i16(&self) -> Option<i16> {
    match self {
      &Value::I16(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_i32(&self) -> Option<i32> {
    match self {
      &Value::I32(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_i64(&self) -> Option<i64> {
    match self {
      &Value::I64(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_f32(&self) -> Option<f32> {
    match self {
      &Value::F32(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_f64(&self) -> Option<f64> {
    match self {
      &Value::F64(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_bool(&self) -> Option<bool> {
    match self {
      &Value::Bool(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_vec(&self) -> Option<Vec<Value<'a>>> {
    match self {
      &Value::Vec(v) => Some(v),
      _ => None,
    }
  }
  pub fn as_hash_map(&self) -> Option<HashMap<String, Value<'a>>> {
    match self {
      &Value::HashMap(v) => Some(v),
      _ => None,
    }
  }
}

impl<'a> Index for Value<'a> {}
