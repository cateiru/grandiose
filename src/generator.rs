use crate::error::Error;
use tiny_skia::{Color, FillRule, Paint, Path, PathBuilder, Pixmap, Transform};

pub struct Generator {
  width: u32,
  height: u32,
  color_stroke: Vec<u8>,
  color_background: Vec<u8>,
  all: u32,
}

impl Generator {
  pub fn new() -> Option<Self> {
    Some(Self {
      width: 1920,
      height: 1080,
      color_stroke: vec![0, 0, 0],
      color_background: vec![255, 255, 255],
      all: 10,
    })
  }

  pub fn create(&self) -> Result<Pixmap, Error> {
    let pixmap = Pixmap::new(self.width, self.height);

    match pixmap {
      Some(mut pixmap) => {
        let background = Color::from_rgba8(
          self.color_background[0],
          self.color_background[1],
          self.color_background[2],
          255,
        );
        pixmap.fill(background);
        let mut paint = Paint::default();

        paint.set_color_rgba8(
          self.color_stroke[0],
          self.color_stroke[1],
          self.color_stroke[2],
          100,
        );

        for rotate in 0..self.all {
          let pb = PathBuilder::new();
          let line = concentrated_line(pb, rotate, self.all);

          if let Some(_line) = line {
            pixmap.fill_path(
              &_line,
              &paint,
              FillRule::Winding,
              Transform::identity(),
              None,
            );
          }
        }
        Ok(pixmap)
      }
      None => Err(Error::DontCreateError("pixmap".to_string())),
    }
  }
}

fn concentrated_line(mut pb: PathBuilder, rotation: u32, all: u32) -> Option<Path> {
  pb.close();
  pb.finish()
}
