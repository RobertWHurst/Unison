mod config;
mod schema;
mod value;

pub use config::Config;
pub use value::Value;

pub fn unison(name: &'static str) -> Config {
  Config::new(name)
}
