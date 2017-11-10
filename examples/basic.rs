extern crate unison;

fn main() {
  let config = unison::Config::load("basic").unwrap();

  println!("{:#?}", config);

  println!("{:?}", config.get("foo.bar").as_str());
  println!("{:?}", config.get("test.foo").as_str());
  println!("{:?}", config.get("test.n").as_u32());
}
