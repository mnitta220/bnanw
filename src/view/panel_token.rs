use super::super::model::token;

pub struct PanelToken {
  pub seq: isize,
  pub ty: token::TokenType,
  pub word: String,
  pub ruby: Option<Vec<PanelToken>>,
  pub width: f64,
}

impl PanelToken {
  pub fn clone(token: &PanelToken) -> Self {
    let mut ruby: Option<Vec<PanelToken>> = None;

    if let Some(rb) = &token.ruby {
      let mut v: Vec<PanelToken> = Vec::new();

      for r in rb {
        v.push(PanelToken::clone(r));
      }

      ruby = Some(v);
    }

    PanelToken {
      seq: token.seq,
      ty: token.ty,
      word: String::from(&token.word),
      ruby: ruby,
      width: token.width,
    }
  }

  pub fn to_string(&self) -> String {
    let mut s = String::from("[");

    if let Some(v) = &self.ruby {
      for r in v {
        s.push_str(&r.to_string());
        s.push_str(", ");
      }
    }

    s.push_str("]");

    format!(
      "PanelToken: seq={} word={} ty={} width={} ruby={}",
      self.seq, self.word, self.ty, self.width, s
    )
  }

  pub fn ruby_len(&self) -> usize {
    let mut w: usize = 0;

    if let Some(rs) = &self.ruby {
      for r in rs {
        w += r.word.chars().count();
      }
    }

    w
  }

  pub fn ruby_width(&self) -> f64 {
    let mut w: f64 = 0.0;

    if let Some(rs) = &self.ruby {
      for r in rs {
        w += r.width;
      }
    }

    w
  }

  pub fn max_width(&self) -> f64 {
    //let rw = self.ruby_width();

    //if rw > self.width {
    //  rw
    //} else {
    self.width
    //}
  }
}
