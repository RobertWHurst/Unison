#![feature(refcell_replace_swap)]

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;

mod config;
mod value;
mod error;

pub use config::{Config, Kind};
pub use value::Value;
pub use error::Error;
