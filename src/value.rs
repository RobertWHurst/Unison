use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
    ops::Index,
};

/// Value is used to organize config data into a hierarchical structure.
/// It contains a variant corresponding to the values available in JSON.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Number contains an i32.
    Number(i32),
    /// Bool contains an bool.
    Bool(bool),
    /// String contains a String.
    String(String),
    /// Object contains a HashMap in which more Value instances can be nested.
    Object(HashMap<String, Value>),
    /// Array contains a Vec in which more Value instances can be nested.
    Array(Vec<Value>),
    /// Null represents the null value from JSON.
    Null,
    /// None represents the absence of a value. It's usually found when indexing
    /// a non existant path.
    None,
}

impl Value {
    /// merge takes a Value instance and merges it into the Value instance its
    /// called upon.
    pub fn merge(&mut self, value: Value) {
        match value {
            Value::Object(source_map) => {
                if let Value::Object(existing_map) = self {
                    for (k, m) in source_map {
                        if let Some(existing_value) = existing_map.get_mut(&k) {
                            existing_value.merge(m);
                        } else {
                            existing_map.insert(k, m);
                        }
                    }
                } else {
                    *self = Value::Object(source_map);
                }
            }
            _ => *self = value,
        }
    }

    /// get_path allows you to traverse nested Object and Array Value variants.
    pub fn get_path(&self, path: &str) -> &Value {
        let mut chunks = path.split(".");
        let mut ctx = Some(self);
        loop {
            let chunk = match chunks.next() {
                Some(c) => c,
                None => break,
            };
            ctx = match ctx {
                Some(Value::Object(m)) => m.get(chunk),
                Some(Value::Array(a)) => match chunk.parse::<usize>() {
                    Ok(i) => a.get(i),
                    Err(_) => None,
                },
                _ => None,
            }
        }
        ctx.unwrap_or(&Value::None)
    }

    /// set_path allows you to traverse nested Object and Array Value variants,
    /// and set/merge a value into the Value instance its called upon at the
    /// given path.
    pub fn set_path(&mut self, path: &str, value: Value) {
        let mut chunks = path.split(".");

        let mut ctx = self;

        loop {
            let chunk = match chunks.next() {
                Some(c) => c,
                None => break,
            };
            ctx = match ctx {
                Value::Object(m) => m
                    .entry(chunk.into())
                    .or_insert_with(|| Value::Object(HashMap::new())),
                Value::Array(a) => match chunk.parse::<usize>() {
                    Ok(i) => a.iter_mut().skip(i).next().unwrap(),
                    Err(_) => break,
                },
                _ => break,
            };
        }
        ctx.merge(value);
    }
}

impl From<i32> for Value {
    fn from(int: i32) -> Self {
        Value::Number(int)
    }
}

impl From<bool> for Value {
    fn from(bool: bool) -> Self {
        Value::Bool(bool)
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::String(string)
    }
}

impl<T: Into<Value>> From<HashMap<String, T>> for Value {
    fn from(map: HashMap<String, T>) -> Self {
        let mut value = HashMap::new();
        for (k, v) in map {
            value.insert(k, v.into());
        }
        Value::Object(value)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(vec: Vec<T>) -> Self {
        let mut value = Vec::new();
        for v in vec {
            value.push(v.into())
        }
        Value::Array(value)
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null
    }
}

impl<'a> Index<&'a str> for Value {
    type Output = Value;

    fn index(&self, path: &'a str) -> &Self::Output {
        self.get_path(path)
    }
}

#[test]
fn can_merge_simple_value() {
    let mut value = Value::None;
    value.merge(Value::Number(23));

    matches!(value, Value::Number(n) if n == 23);
}

#[test]
fn can_merge_map_value() {
    let mut map_a_1 = HashMap::new();
    map_a_1.insert(
        "alpha".to_string(),
        Value::Array(vec![Value::Number(1), Value::Number(2), Value::Number(3)]),
    );
    map_a_1.insert("beta".to_string(), Value::Null);

    let mut map_a = HashMap::new();
    map_a.insert("foo".to_string(), Value::Bool(true));
    map_a.insert("bar".to_string(), Value::Number(23));
    map_a.insert("baz".to_string(), Value::Object(map_a_1));
    map_a.insert("ack".to_string(), Value::String("cat".into()));

    let mut map_b_1 = HashMap::new();
    map_b_1.insert(
        "alpha".to_string(),
        Value::Array(vec![Value::Number(3), Value::Number(4)]),
    );
    map_b_1.insert("beta".to_string(), Value::String("second".into()));

    let mut map_b = HashMap::new();
    map_b.insert("foo".to_string(), Value::Bool(false));
    map_b.insert("baz".to_string(), Value::Object(map_b_1));
    map_b.insert("ack".to_string(), Value::String("dog".into()));
    map_b.insert("biz".to_string(), Value::String("wheel".into()));

    let value = Value::Object(map_a);

    matches!(&value, Value::Object(m) if m.len() == 5);
    if let Value::Object(map) = value {
        matches!(map.get("foo"), Some(Value::Bool(b)) if *b == false);
        matches!(map.get("bar"), Some(Value::Number(n)) if *n == 23);
        matches!(map.get("baz"), Some(Value::Object(m)) if m.len() == 2);
        matches!(map.get("ack"), Some(Value::String(s)) if s == "dog");
        matches!(map.get("biz"), Some(Value::String(s)) if s == "wheel");

        let value = map.get("baz").unwrap();
        if let Value::Object(map) = value {
            matches!(map.get("alpha"), Some(Value::Array(v)) if v.len() == 3);
            matches!(map.get("beta"), Some(Value::String(v)) if v == "second");
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    }
}

#[test]
fn can_create_from_i32() {
    matches!(Value::from(23), Value::Number(n) if n == 23);
}

#[test]
fn can_create_from_bool() {
    matches!(Value::from(true), Value::Bool(n) if n == true);
}

#[test]
fn can_create_from_string() {
    matches!(Value::from("truck".to_string()), Value::String(s) if s == "truck");
}

#[test]
fn can_create_from_map() {
    let mut map = HashMap::new();
    map.insert("first".to_string(), 10);
    map.insert("second".to_string(), 20);
    map.insert("third".to_string(), 30);

    let value = Value::from(map);

    matches!(&value, Value::Object(m) if m.len() == 3);
    if let Value::Object(map) = value {
        matches!(map.get("first"), Some(Value::Number(n)) if *n == 10);
        matches!(map.get("second"), Some(Value::Number(n)) if *n == 20);
        matches!(map.get("third"), Some(Value::Number(n)) if *n == 30);
    } else {
        unreachable!();
    }
}

#[test]
fn can_create_from_vec() {
    let vec = vec![1, 2, 3];
    let value = Value::from(vec);
    matches!(&value, Value::Array(v) if v.len() == 3);
    if let Value::Array(v) = value {
        matches!(v[0], Value::Number(n) if n == 1);
        matches!(v[1], Value::Number(n) if n == 2);
        matches!(v[2], Value::Number(n) if n == 3);
    }
}

#[test]
fn can_create_from_empty_tuple() {
    let value = Value::from(());
    matches!(value, Value::Null);
}

#[test]
fn can_be_path_indexed() {
    let mut foo = HashMap::new();
    let mut bar = HashMap::new();
    let mut baz = HashMap::new();
    baz.insert("ack".to_string(), 1);
    bar.insert("baz".to_string(), baz);
    foo.insert("bar".to_string(), bar);

    let value = Value::from(foo);

    let num = &value["bar.baz.ack"];
    let none = &value["foo"];

    assert_eq!(*num, Value::Number(1));
    assert_eq!(*none, Value::None);
}
