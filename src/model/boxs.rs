pub struct Box1 {
  pub is_sec: bool,
  pub section: isize,
  pub label: char,
}

pub struct BoxLine {
  pub boxs: Vec<Box1>,
  pub cur: i32,
}

pub struct Box9 {
  pub lines: Vec<BoxLine>,
  pub cur: i32,
}

impl Box1 {
  pub fn new(is_sec: bool, section: isize, label: char) -> Self {
    let b = Box1 {
      is_sec: is_sec,
      section: section,
      label: label,
    };

    b
  }
}

impl BoxLine {
  pub fn new() -> Self {
    let l = BoxLine {
      boxs: Vec::new(),
      cur: 0,
    };

    l
  }
}

impl Box9 {
  pub fn new() -> Self {
    let b = Box9 {
      lines: Vec::new(),
      cur: 0,
    };

    b
  }
}
