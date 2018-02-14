use std::cmp::Eq;
use std::mem;
use std::hash::Hash;
use std::str::FromStr;
use std::collections::HashMap;
use failure::{Error, Fail};
use schema::Schema;
use config::ConfigError;

/// A enum value used to contain configuration values.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Bool(bool),

  USize(usize),
  ISize(isize),

  U8(u8),
  U16(u16),
  U32(u32),
  U64(u64),
  #[cfg(feature = "int128")] U128(u128),

  I8(i8),
  I16(i16),
  I32(i32),
  I64(i64),
  #[cfg(feature = "int128")] I128(i128),

  F32(f32),
  F64(f64),

  String(String),

  HashMap(HashMap<String, Value>),
  Vec(Vec<Value>),

  None,
}

impl Value {
  /// Creates a value tree from a given raw string value, path, and schema.
  /// The schema is used to cast the raw value to the correct type.
  pub fn from_raw_with_schema_and_path(
    value: Option<String>,
    path_name: &str,
    schema: &Schema,
  ) -> Result<Value, ConfigError> {
    match value {
      // bool
      None if schema.path_is_type::<bool>(path_name) => Ok(Value::Bool(true)),

      // word sized ints
      Some(ref v) if schema.path_is_type::<usize>(path_name) => Ok(Value::USize(v.parse()
        .map_err(|_| ConfigError::BadPathType(path_name.to_owned(), "usize"))?)),
      Some(ref v) if schema.path_is_type::<isize>(path_name) => Ok(Value::ISize(v.parse()
        .map_err(|_| ConfigError::BadPathType(path_name.to_owned(), "isize"))?)),

      // unsigned ints
      Some(ref v) if schema.path_is_type::<u8>(path_name) => Ok(Value::U8(v.parse()
        .map_err(|_| ConfigError::BadPathType(path_name.to_owned(), "u8"))?)),
      Some(ref v) if schema.path_is_type::<u16>(path_name) => Ok(Value::U16(v.parse()
        .map_err(|_| ConfigError::BadPathType(path_name.to_owned(), "u16"))?)),
      Some(ref v) if schema.path_is_type::<u32>(path_name) => Ok(Value::U32(v.parse()
        .map_err(|_| ConfigError::BadPathType(path_name.to_owned(), "u32"))?)),
      Some(ref v) if schema.path_is_type::<u64>(path_name) => Ok(Value::U64(v.parse()
        .map_err(|_| ConfigError::BadPathType(path_name.to_owned(), "u64"))?)),
      #[cfg(feature = "int128")]
      Some(ref v) if schema.path_is_type::<u128>(path_name) =>
      {
        Ok(Value::U128(v.parse().map_err(|_| {
          ConfigError::BadPathType(path_name.to_owned(), "u128")
        })?))
      }

      // signed ints
      Some(ref v) if schema.path_is_type::<i8>(path_name) => Ok(Value::I8(v.parse()
        .map_err(|_| ConfigError::BadPathType(path_name.to_owned(), "i8"))?)),
      Some(ref v) if schema.path_is_type::<i16>(path_name) => Ok(Value::I16(v.parse()
        .map_err(|_| ConfigError::BadPathType(path_name.to_owned(), "i16"))?)),
      Some(ref v) if schema.path_is_type::<i32>(path_name) => Ok(Value::I32(v.parse()
        .map_err(|_| ConfigError::BadPathType(path_name.to_owned(), "i32"))?)),
      Some(ref v) if schema.path_is_type::<i64>(path_name) => Ok(Value::I64(v.parse()
        .map_err(|_| ConfigError::BadPathType(path_name.to_owned(), "i64"))?)),
      #[cfg(feature = "int128")]
      Some(ref v) if schema.path_is_type::<i128>(path_name) =>
      {
        Ok(Value::I128(v.parse().map_err(|_| {
          ConfigError::BadPathType(path_name.to_owned(), "i128")
        })?))
      }

      // floats
      Some(ref v) if schema.path_is_type::<f32>(path_name) => Ok(Value::F32(v.parse()
        .map_err(|_| ConfigError::BadPathType(path_name.to_owned(), "f32"))?)),
      Some(ref v) if schema.path_is_type::<f64>(path_name) => Ok(Value::F64(v.parse()
        .map_err(|_| ConfigError::BadPathType(path_name.to_owned(), "f64"))?)),

      // String
      Some(ref v) if schema.path_is_type::<String>(path_name) => Ok(Value::String(v.to_owned())),

      // Collections
      Some(_) if schema.path_is_type::<HashMap<String, Value>>(path_name) => unimplemented!(),
      Some(_) if schema.path_is_type::<Vec<Value>>(path_name) => unimplemented!(),

      // None
      Some(ref v) if v == "_" => Ok(Value::None),

      _ => Err(ConfigError::BadPathType(path_name.to_owned(), "_")),
    }
  }

  /// Gets the value at a given path relative the value instance get_path is
  /// called upon.
  pub fn get_path<P: AsRef<str>>(&self, path: P) -> &Value {
    let path: Vec<_> = path.as_ref().split(".").collect();
    let mut ctx = self;
    for key in path {
      ctx = ctx.get_key(key);
      if let Value::None = *ctx {
        break;
      }
    }
    ctx
  }

  fn get_key(&self, key: &str) -> &Value {
    match *self {
      Value::HashMap(ref h) => h.get(key).unwrap_or(&Value::None),
      _ => &Value::None,
    }
  }

  fn get_key_mut(&mut self, key: &str) -> Option<&mut Value> {
    match *self {
      Value::HashMap(ref mut h) => h.get_mut(key),
      _ => None,
    }
  }

  /// Sets a value at a given path creating value nodes in the tree as
  /// nessisary.
  pub fn set_path<P: AsRef<str>>(&mut self, path: P, mut value: Value) {
    let mut path = path.as_ref();

    if let Some(key_end_index) = path.find(".") {
      let next_key = &path[0..key_end_index];

      if let Some(next_value) = self.get_key_mut(next_key) {
        let trailing_path = &path[key_end_index..];
        return next_value.set_path(trailing_path, value);
      }

      let mut keys: Vec<_> = path.split(".").collect();
      keys.reverse();
      path = keys.pop().unwrap();

      for key in keys {
        let mut hash_map = HashMap::new();
        hash_map.insert(key.to_owned(), value);
        value = Value::HashMap(hash_map);
      }
    }

    self.set_key(path, value);
  }

  fn set_key<K: Into<String>>(&mut self, key: K, value: Value) {
    match *self {
      Value::HashMap(ref mut h) => {
        h.insert(key.into(), value);
      }
      ref mut v => {
        let mut values = Value::HashMap(HashMap::new());
        values.set_key(key, value);
        mem::replace(v, values);
      }
    };
  }

  /// Casts the current value into the type argument for `T`.
  /// `T` must implement `FromValue`.
  pub fn into_type<T: FromValue>(self) -> Result<T, FromValueError> {
    T::from_value(self)
  }
}

/// FromValue allows casting a value into any type that implements it.
pub trait FromValue: Sized {
  fn from_value(Value) -> Result<Self, FromValueError>;
}

/// IntoValue allows casting any type that implements it into a value.
pub trait IntoValue {
  fn into_value(self) -> Value;
}

impl FromValue for Value {
  fn from_value(value: Value) -> Result<Self, FromValueError> {
    Ok(value)
  }
}

impl IntoValue for Value {
  fn into_value(self) -> Value {
    self
  }
}

macro_rules! define_ints {
  ($type:ty, $variant:expr) => {
    impl FromValue for $type {
      fn from_value(value: Value) -> Result<Self, FromValueError> {
        match value {
          Value::USize(n) => Ok(n as $type),
          Value::ISize(n) => Ok(n as $type),

          Value::U8(n) => Ok(n as $type),
          Value::U16(n) => Ok(n as $type),
          Value::U32(n) => Ok(n as $type),
          Value::U64(n) => Ok(n as $type),
          #[cfg(feature = "int128")]
          Value::U128(n) => Ok(n as $type),

          Value::I8(n) => Ok(n as $type),
          Value::I16(n) => Ok(n as $type),
          Value::I32(n) => Ok(n as $type),
          Value::I64(n) => Ok(n as $type),
          #[cfg(feature = "int128")]
          Value::I128(n) => Ok(n as $type),

          Value::F32(n) => Ok(n as $type),
          Value::F64(n) => Ok(n as $type),
          _ => Err(FromValueError::InvalidCast(
            format_err!("Cannot cast non-number type value into `{}`.", stringify!($variant))
          )),
        }
      }
    }

    impl IntoValue for $type {
      fn into_value(self) -> Value {
        $variant(self)
      }
    }
  };
}

impl<'a> FromValue for bool {
  fn from_value(value: Value) -> Result<Self, FromValueError> {
    match value {
      Value::Bool(b) => Ok(b),
      _ => Err(FromValueError::InvalidCast(format_err!(
        "Cannot cast non-boolean value into `bool`."
      ))),
    }
  }
}

impl<'a> IntoValue for bool {
  fn into_value(self) -> Value {
    Value::Bool(self)
  }
}

define_ints!(usize, Value::USize);
define_ints!(isize, Value::ISize);

define_ints!(u8, Value::U8);
define_ints!(u16, Value::U16);
define_ints!(u32, Value::U32);
define_ints!(u64, Value::U64);
#[cfg(feature = "int128")]
define_ints!(u128, Value::U128);

define_ints!(i8, Value::I8);
define_ints!(i16, Value::I16);
define_ints!(i32, Value::I32);
define_ints!(i64, Value::I64);
#[cfg(feature = "int128")]
define_ints!(i128, Value::I128);

define_ints!(f32, Value::F32);
define_ints!(f64, Value::F64);

impl<'a> FromValue for String {
  fn from_value(value: Value) -> Result<Self, FromValueError> {
    match value {
      Value::String(s) => Ok(s),
      _ => Err(FromValueError::InvalidCast(format_err!(
        "Cannot cast non-string value into `String`."
      ))),
    }
  }
}

impl<'a> IntoValue for String {
  fn into_value(self) -> Value {
    Value::String(self)
  }
}

impl<'a> IntoValue for &'a str {
  fn into_value(self) -> Value {
    Value::String(self.to_owned())
  }
}

impl<K, T, E> FromValue for HashMap<K, T>
where
  K: FromStr<Err = E> + Eq + Hash,
  T: FromValue,
  E: Fail,
{
  fn from_value(value: Value) -> Result<Self, FromValueError> {
    let value_hash_map = match value {
      Value::HashMap(h) => h,
      _ => {
        return Err(FromValueError::InvalidCast(format_err!(
          "Cannot cast non hash map value into `HashMap`."
        )))
      }
    };

    let mut hash_map = HashMap::new();
    for (key, value) in value_hash_map {
      let key = key
        .parse()
        .map_err(|e| FromValueError::KeyParseError(Error::from(e)))?;
      let value = T::from_value(value)?;
      hash_map.insert(key, value);
    }

    Ok(hash_map)
  }
}

impl<K: Into<String> + Eq + Hash, T: IntoValue> IntoValue for HashMap<K, T> {
  fn into_value(self) -> Value {
    let mut hash_map = HashMap::new();
    for (key, value) in self {
      let key = key.into();
      let value = value.into_value();
      hash_map.insert(key, value);
    }
    Value::HashMap(hash_map)
  }
}

impl<T: FromValue> FromValue for Vec<T> {
  fn from_value(value: Value) -> Result<Self, FromValueError> {
    let value_vec = match value {
      Value::Vec(h) => h,
      _ => {
        return Err(FromValueError::InvalidCast(format_err!(
          "Cannot convert non-vec value into `Vec`."
        )))
      }
    };

    let mut vec = Vec::new();
    for value in value_vec {
      vec.push(T::from_value(value)?);
    }

    Ok(vec)
  }
}

impl<T: IntoValue> IntoValue for Vec<T> {
  fn into_value(self) -> Value {
    let mut vec = Vec::new();
    for value in self {
      vec.push(value.into_value());
    }
    Value::Vec(vec)
  }
}

#[derive(Debug, Fail)]
#[fail(display = "Missing path")]
pub struct MissingPathError;

#[derive(Debug, Fail)]
pub enum FromValueError {
  #[fail(display = "Cannot cast value: {}", _0)] InvalidCast(#[fail(cause)] Error),
  #[fail(display = "Cannot parse key: {}", _0)] KeyParseError(#[fail(cause)] Error),
}
