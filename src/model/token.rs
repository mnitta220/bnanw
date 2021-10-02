use std::fmt;

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
  Zenkaku,
  Zenkigo,
  Kana,
  Yousoku, // 拗促音
  Alpha,
  Hankigo,
  Kuten,
  Space,
  Tatebo,
  Slash,
  RubyS,
  RubyE,
  Special,
  None,
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      TokenType::Zenkaku => write!(f, "Zenkaku"),
      TokenType::Zenkigo => write!(f, "Zenkigo"),
      TokenType::Kana => write!(f, "Kana"),
      TokenType::Yousoku => write!(f, "Yousoku"),
      TokenType::Alpha => write!(f, "Alpha"),
      TokenType::Hankigo => write!(f, "Hankigo"),
      TokenType::Kuten => write!(f, "Kuten"),
      TokenType::Space => write!(f, "Space"),
      TokenType::Tatebo => write!(f, "Tatebo"),
      TokenType::Slash => write!(f, "Slash"),
      TokenType::RubyS => write!(f, "RubyS"),
      TokenType::RubyE => write!(f, "RubyE"),
      TokenType::Special => write!(f, "Special"),
      TokenType::None => write!(f, "None"),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Token {
  pub ty: TokenType,
  pub word: String,
}

impl Token {
  pub fn new(ty: TokenType, word: &str) -> Self {
    Token {
      ty: ty,
      word: String::from(word),
    }
  }

  pub fn to_string(&self) -> String {
    format!("Token: word={} ty={}", self.word, self.ty)
  }
}

#[derive(Clone, Debug)]
pub struct Token2 {
  pub seq: isize,
  pub ty: TokenType,
  pub word: String,
  pub ruby: Option<Vec<Token2>>,
}

impl Token2 {
  pub fn clone(token: &Token2) -> Self {
    let mut ruby: Option<Vec<Token2>> = None;

    if let Some(rb) = &token.ruby {
      let mut v: Vec<Token2> = Vec::new();

      for r in rb {
        v.push(Token2::clone(r));
      }

      ruby = Some(v);
    }

    Token2 {
      seq: token.seq,
      ty: token.ty,
      word: String::from(&token.word),
      ruby: ruby,
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
      "PanelToken: seq={} word={} ty={} ruby={}",
      self.seq, self.word, self.ty, s
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
}
