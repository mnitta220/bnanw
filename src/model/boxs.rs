use super::source;
use super::token;

pub struct Box1 {
  pub is_sec: bool,
  pub section: isize,
  pub label: String,
}

pub struct BoxLine {
  pub boxs: Vec<Box1>,
  pub cur: i32,
  pub is_end: bool,
}

pub struct Box9 {
  pub lines: Vec<BoxLine>,
  pub cur: i32,
  pub is_end: bool,
}

impl Box1 {
  pub fn new(is_sec: bool, section: isize, label: String) -> Self {
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
      is_end: false,
    };

    l
  }

  /// トークンを追加する。
  ///
  /// # 引数
  /// ## source
  /// ## token
  /// ## pos
  ///
  /// # 戻り値
  /// - >0 : 戻り値の位置までtokenが入った
  /// - -1 : tokenがすべて入った
  pub fn add_token(&self, source: &source::Source, token: &token::Token, pos: isize) -> isize {
    //format!("Token: word={} ty={}", self.word, self.ty)
    //let mut bl = BoxLine::new();
    let mut p = pos;

    match token.ty {
      token::TokenType::Zenkaku
      | token::TokenType::Zenkigo
      | token::TokenType::Kana
      | token::TokenType::Yousoku
      | token::TokenType::Hankigo => {
        if self.boxs.len() > 2 {
          self.is_end = true;
        } else {
          let mut i = 0;
          for c in token.word.chars() {
            if i >= pos {
              let b1 = Box1::new(false, source.seq, String::from(c));
              self.boxs.push(b1);
              p = i;
              if self.boxs.len() > 2 {
                break;
              }
            }
            i += 1;
          }
        }
      }
      token::TokenType::Alpha => {
        if self.boxs.len() == 0 {
          let b1 = Box1::new(false, source.seq, token.word);
          self.boxs.push(b1);
        }
        self.is_end = true;
      }
      token::TokenType::Kuten => {
        let mut i = 0;
        for c in token.word.chars() {
          if i >= pos {
            let b1 = Box1::new(false, source.seq, String::from(c));
            self.boxs.push(b1);
            p = i;
            if self.boxs.len() > 2 {
              break;
            }
          }
          i += 1;
        }
      }
      _ => {}
    }

    p
  }
}

impl Box9 {
  pub fn new() -> Self {
    let b = Box9 {
      lines: Vec::new(),
      cur: 0,
      is_end: false,
    };
    //let bl = BoxLine::new();
    b.lines.push(BoxLine::new());

    b
  }

  /// トークンを追加する。
  ///
  /// # 引数
  /// ## source
  /// ## token
  /// ## pos
  ///
  /// # 戻り値
  /// - >0 : 戻り値の位置までtokenが入った
  /// - -1 : tokenがすべて入った
  pub fn add_token(&self, source: &source::Source, token: &token::Token, pos: isize) -> isize {
    //format!("Token: word={} ty={}", self.word, self.ty)
    let mut is_added = false;
    let mut p = pos;

    for bl in self.lines {
      if bl.is_end == false {
        p = bl.add_token(source, token, p);
        is_added = true;
        break;
      }
    }

    if is_added == false {
      if self.lines.len() > 2 {
        self.is_end = true;
      } else {
        let bl = BoxLine::new();
        p = bl.add_token(source, token, p);
        self.lines.push(bl);
      }
    }

    p
  }
}
