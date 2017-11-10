extern crate unison;

fn main() {
  let config = unison::Config::load("basic").unwrap();

  println!("{:?}", config.get("test.foo").as_str());
  println!("{:?}", config.get("test.n").as_u32());
  println!("{:?}", config.get("data.2.x").as_u32());
}
