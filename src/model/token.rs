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
