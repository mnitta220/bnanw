pub struct Area {
  pub source: isize,
  pub token: isize,
  pub x1: f64,
  pub y1: f64,
  pub x2: f64,
  pub y2: f64,
}

impl Area {
  pub fn new(source: isize, token: isize, x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
    Area {
      source: source,
      token: token,
      x1: x1,
      y1: y1,
      x2: x2,
      y2: y2,
    }
  }

  pub fn in_area(&self, x: f64, y: f64) -> bool {
    if self.x1 <= x && x <= self.x2 && self.y1 <= y && y <= self.y2 {
      true
    } else {
      false
    }
  }

  pub fn touch_pos(areas: &Vec<Area>, x: f64, y: f64) -> (isize, isize) {
    for a in areas {
      //log!("***touch_pos: x={} y={} {}", x, y, a.to_string());
      if a.in_area(x, y) {
        return (a.source, a.token);
      }
    }

    (-3, -3)
  }

  pub fn to_string(&self) -> String {
    format!(
      "Area: source={} token={} x1={} y1={} x2={} y2={}",
      self.source, self.token, self.x1, self.y1, self.x2, self.y2
    )
  }
}
