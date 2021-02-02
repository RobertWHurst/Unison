#![warn(missing_docs)]
#![warn(missing_crate_level_docs)]

mod config;
mod schema;
mod value;

pub use config::Config;
pub use value::Value;

pub fn unison(name: &'static str) -> Config {
    Config::new(name)
}
