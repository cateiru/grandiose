use grandiose::Generator;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let a = Generator::new().unwrap();
  let pixmap = a.create().unwrap();
  pixmap.save_png("a.png").unwrap();

  Ok(())
}
