use grandiose::{save, Generator};
use std::error::Error;
use tiny_skia::Pixmap;

fn main() -> Result<(), Box<dyn Error>> {
  let a = Generator::new().unwrap();
  let pixmap = a.create().unwrap();
  save(pixmap, "a.png");

  Ok(())
}
