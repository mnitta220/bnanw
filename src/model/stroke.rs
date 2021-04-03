
pub struct Point {
  pub x: i32,
  pub y: i32,
}

pub struct Stroke {
  pub points: Vec<Point>,
}

impl Point {
  pub fn new(x: i32, y: i32) -> Self {
    Point {
      x: x,
      y: y,
    }
  }
}

impl Stroke {
  pub fn new() -> Self {
    Stroke {
      points: Vec::new(),
    }
  }
}
