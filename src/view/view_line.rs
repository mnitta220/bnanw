use super::super::model::source;
use super::panel_token;

pub struct ViewLine {
  pub first: bool,
  pub last: bool,
  pub align: source::Align,
  pub count: i32,
  pub width: f64,
  pub ptokens: Vec<panel_token::PanelToken>,
  pub first_token_idx: usize,
}

impl ViewLine {
  pub fn new() -> Self {
    ViewLine {
      first: false,
      last: false,
      align: source::Align::None,
      count: 0,
      width: 0.0,
      ptokens: Vec::new(),
      first_token_idx: 0,
    }
  }

  pub fn to_string(&self) -> String {
    format!(
      "ViewLine: first={} last={} align={} count={} width={} first_token_idx={}",
      self.first, self.last, self.align, self.count, self.width, self.first_token_idx
    )
  }
}
