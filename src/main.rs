use grandiose::generate;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  generate();
  Ok(())
}
