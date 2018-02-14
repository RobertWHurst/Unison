extern crate unison;

use unison::*;

fn main() {
  let schema = Schema::new().path("server.port", 8000).build();

  let config = Config::load("basic", schema).unwrap();

  let port: u32 = config.get("server.port");

  println!("PORT {}", port);
}
