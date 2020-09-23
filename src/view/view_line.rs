use super::super::model::source;
use super::panel_token;

pub struct ViewLine {
  //pub start: isize,
  //pub end: isize,
  pub last: bool,
  pub align: source::Align,
  pub count: i32,
  pub width: f64,
  pub ptokens: Vec<panel_token::PanelToken>,
}

impl ViewLine {
  pub fn new() -> Self {
    ViewLine {
      //start: 0,
      //end: 0,
      last: false,
      align: source::Align::None,
      count: 0,
      width: 0.0,
      ptokens: Vec::new(),
    }
  }

  pub fn to_string(&self) -> String {
    /*
    format!(
      "ViewLine: start={} end={} last={} align={} count={} width={}",
      self.start, self.end, self.last, self.align, self.count, self.width
    )
    */
    format!(
      "ViewLine: last={} align={} count={} width={}",
      self.last, self.align, self.count, self.width
    )
  }
}
