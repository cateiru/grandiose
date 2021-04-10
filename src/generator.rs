use crate::error::Error;
use tiny_skia::{FillRule, Paint, Path, PathBuilder, Pixmap, Transform};

pub struct Generator {
  width: u32,
  height: u32,
  color_r: u8,
  color_g: u8,
  color_b: u8,
}

impl Generator {
  pub fn new() -> Option<Self> {
    Some(Self {
      width: 1920,
      height: 1080,
      color_r: 225,
      color_g: 225,
      color_b: 225,
    })
  }

  pub fn create(&self) -> Result<Pixmap, Error> {
    let mut pixmap = Pixmap::new(self.width, self.height);

    match pixmap {
      Some(_pixmap) => {
        let pixmap = view(self.color_r, self.color_g, self.color_b, _pixmap);
        Ok(pixmap)
      }
      None => Err(Error::DontCreateError("pixmap".to_string())),
    }
  }
}

pub fn save<P: AsRef<std::path::Path>>(pixmap: Pixmap, save_path: P) {
  pixmap.save_png(save_path);
}

fn view(r: u8, g: u8, b: u8, mut pixmap: Pixmap) -> Pixmap {
  let mut paint = Paint::default();
  let mut pb = PathBuilder::new();

  paint.set_color_rgba8(r, g, b, 200);
  let line = concentrated_line(pb);

  if let Some(_line) = line {
    pixmap.fill_path(
      &_line,
      &paint,
      FillRule::Winding,
      Transform::identity(),
      None,
    );
  }

  pixmap
}

fn concentrated_line(mut pb: PathBuilder) -> Option<Path> {
  pb.move_to(60.0, 60.0);
  pb.line_to(160.0, 940.0);
  pb.cubic_to(380.0, 840.0, 660.0, 800.0, 940.0, 800.0);
  pb.cubic_to(740.0, 460.0, 440.0, 160.0, 60.0, 60.0);
  pb.close();
  pb.finish()
}
