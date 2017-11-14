extern crate unison;

fn main() {
  let config = unison::Config::load("basic").unwrap();

  println!("{:#?}", config);
}
