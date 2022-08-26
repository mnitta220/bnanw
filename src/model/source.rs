use super::token;
use std::fmt;

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub enum Align {
  // 中寄せ
  Center,
  // 地付き（横書きなら右寄せ、縦書きなら下寄せ）
  Bottom,
  // なし
  None,
}

impl fmt::Display for Align {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Align::Center => write!(f, "Center"),
      Align::Bottom => write!(f, "Bottom"),
      Align::None => write!(f, "None"),
    }
  }
}

/*
pub struct BoxLine {
  pub token1: isize,
  pub word1: isize,
  pub token2: isize,
  pub word2: isize,
}

impl BoxLine {
  pub fn new(token1: isize, word1: isize, token2: isize, word2: isize) -> Self {
    let l = BoxLine {
      token1,
      word1,
      token2,
      word2,
    };

    l
  }
}
*/

pub struct Source {
  pub seq: isize,
  pub ty: isize,
  pub text: String,
  pub align: Align,
  pub tokens: Vec<token::Token>,
  pub token2s: Vec<token::Token2>,
  //pub box_lines: Vec<BoxLine>,
}

impl Source {
  pub fn new(seq: isize, text: &str) -> Self {
    let mut s = Source {
      seq: seq,
      ty: 0,
      text: String::from(text),
      align: Align::None,
      tokens: Vec::new(),
      token2s: Vec::new(),
      //box_lines: Vec::new(),
    };

    s.tokenize();
    s.analyze();
    //s.sprit_box_line();

    s
  }

  fn tokenize(&mut self) {
    //log!("***tokenize");
    let mut token: token::Token;
    let mut buf: String = String::new();
    let mut buf_type: token::TokenType = token::TokenType::None;
    let mut t: isize = 0;
    let mut c1 = ' ';
    let mut i = -1;

    for c in self.text.chars() {
      i += 1;

      // 改行コードをスキップ
      if c as u32 == 10 || c as u32 == 13 {
        continue;
      }

      // 行頭の#は見出し
      if c == '#' && t > -1 {
        t += 1;
      } else {
        if 0 < t && t < 7 {
          self.ty = t;
          self.tokens.clear();
          buf = String::new();
          buf_type = token::TokenType::None;

          if c == ' ' {
            t = -1;
            continue;
          }
        }

        t = -1;
      }

      match c {
        ' ' | '　' => {
          if buf_type != token::TokenType::None {
            token = token::Token::new(buf_type, buf.as_ref());
            self.tokens.push(token);
            buf = String::new();
          }

          buf_type = token::TokenType::Space;
          buf.push(c);
        }

        '/' => {
          if c1 == '/' && buf_type == token::TokenType::Slash {
            // '/'が２つ続いた
            buf_type = token::TokenType::Alpha;
          } else {
            if buf_type != token::TokenType::None {
              token = token::Token::new(buf_type, buf.as_ref());
              self.tokens.push(token);
              buf = String::new();
            }

            buf_type = token::TokenType::Slash;
            buf.push(c);
          }
        }

        '《' => {
          if c1 == '《' && buf_type == token::TokenType::RubyS {
            // '《'が２つ続いた
            buf_type = token::TokenType::Zenkigo;
          } else {
            if buf_type != token::TokenType::None {
              token = token::Token::new(buf_type, buf.as_ref());
              self.tokens.push(token);
              buf = String::new();
            }

            buf_type = token::TokenType::RubyS;
            buf.push(c);
          }
        }

        '》' => {
          if i == 0 {
            // 行頭に 》があれば、地付きとする。
            self.align = Align::Bottom;
          } else if i == 1 && c1 == '《' {
            // 行頭に 《》があれば、中寄せとする。
            self.align = Align::Center;
          } else if c1 == '》' && buf_type == token::TokenType::RubyE {
            // '》'が２つ続いた
            buf_type = token::TokenType::Zenkigo;
          } else {
            if buf_type != token::TokenType::None {
              token = token::Token::new(buf_type, buf.as_ref());
              self.tokens.push(token);
              buf = String::new();
            }

            buf_type = token::TokenType::RubyE;
            buf.push(c);
          }
        }

        '｜' | '|' => {
          if c1 == '｜' && buf_type == token::TokenType::Tatebo {
            // '｜'が２つ続いた
            buf_type = token::TokenType::Zenkigo;
          } else if c1 == '|' && buf_type == token::TokenType::Tatebo {
            // '|'が２つ続いた
            buf_type = token::TokenType::Alpha;
          } else {
            if buf_type != token::TokenType::None {
              token = token::Token::new(buf_type, buf.as_ref());
              self.tokens.push(token);
              buf = String::new();
            }

            buf_type = token::TokenType::Tatebo;
            buf.push(c);
          }
        }

        '。' | '、' | '，' | '．' => {
          if buf_type != token::TokenType::None {
            token = token::Token::new(buf_type, buf.as_ref());
            self.tokens.push(token);
            buf = String::new();
          }

          buf_type = token::TokenType::Kuten;
          buf.push(c);
        }

        // ひらがな
        'ぁ'..='・' => match c {
          'ぁ' | 'ぃ' | 'ぅ' | 'ぇ' | 'ぉ' | 'っ' | 'ゃ' | 'ゅ' | 'ょ' | 'ァ' | 'ィ' | 'ゥ'
          | 'ェ' | 'ォ' | 'ッ' | 'ャ' | 'ュ' | 'ョ' => {
            if buf_type != token::TokenType::Yousoku && buf_type != token::TokenType::None {
              token = token::Token::new(buf_type, buf.as_ref());
              self.tokens.push(token);
              buf = String::new();
            }

            buf_type = token::TokenType::Yousoku;
            buf.push(c);
          }
          _ => {
            if buf_type != token::TokenType::Kana && buf_type != token::TokenType::None {
              token = token::Token::new(buf_type, buf.as_ref());
              self.tokens.push(token);
              buf = String::new();
            }

            buf_type = token::TokenType::Kana;
            buf.push(c);
          }
        },

        // 全角記号
        '「'
        | '」'
        | '『'
        | '』'
        | '（'
        | '）'
        | '【'
        | '】'
        | '［'
        | '］'
        | '｛'
        | '｝'
        | '…'
        | '─'
        | '━'
        | 'ー'
        | '＝'
        | '～'
        | '：'
        | '＜'
        | '＞'
        | '←'..='⇿' => {
          if buf_type != token::TokenType::Zenkigo && buf_type != token::TokenType::None {
            token = token::Token::new(buf_type, buf.as_ref());
            self.tokens.push(token);
            buf = String::new();
          }

          buf_type = token::TokenType::Zenkigo;
          buf.push(c);
        }

        // 特殊文字
        '‐'..='₵' | '⃞'..='⚲' => {
          if buf_type != token::TokenType::Special && buf_type != token::TokenType::None {
            token = token::Token::new(buf_type, buf.as_ref());
            self.tokens.push(token);
            buf = String::new();
          }

          buf_type = token::TokenType::Special;
          buf.push(c);
        }

        // 半角記号
        '(' | ')' | '[' | ']' | '{' | '}' => {
          if buf_type != token::TokenType::Hankigo && buf_type != token::TokenType::None {
            token = token::Token::new(buf_type, buf.as_ref());
            self.tokens.push(token);
            buf = String::new();
          }

          buf_type = token::TokenType::Hankigo;
          buf.push(c);
        }

        _ => {
          //if c <= 'ߺ' {
          if c <= 'ʸ' {
            // 半角文字
            if buf_type != token::TokenType::Alpha && buf_type != token::TokenType::None {
              token = token::Token::new(buf_type, buf.as_ref());
              self.tokens.push(token);
              buf = String::new();
            }

            buf_type = token::TokenType::Alpha;
            buf.push(c);
          } else {
            // 全角文字
            if buf_type != token::TokenType::Zenkaku && buf_type != token::TokenType::None {
              token = token::Token::new(buf_type, buf.as_ref());
              self.tokens.push(token);
              buf = String::new();
            }

            buf_type = token::TokenType::Zenkaku;
            buf.push(c);
          }
        }
      }

      c1 = c;
    }

    if buf_type != token::TokenType::None {
      token = token::Token::new(buf_type, buf.as_ref());
      self.tokens.push(token);
    }
  }

  fn analyze(&mut self) {
    //log!("***analyze");
    let mut i: usize = 0;
    let mut j: usize;
    let mut ruby_s: usize;
    let mut ruby_e: usize;
    //let mut ruby_len: i32;
    let mut ruby_tokens: Vec<token::Token2>;
    let mut t: token::Token2;
    let mut seq = 0;

    loop {
      if i >= self.tokens.len() {
        break;
      }

      ruby_s = i;
      ruby_e = i;
      //ruby_len = 0;
      ruby_tokens = Vec::new();

      if (i + 1) < self.tokens.len() {
        // ルビがあるか？
        j = i + 1;

        if self.tokens[j].ty == token::TokenType::RubyS {
          ruby_s = j;
          j += 1;

          loop {
            if j >= self.tokens.len() {
              break;
            }

            match self.tokens[j].ty {
              token::TokenType::RubyE => {
                ruby_e = j;
                break;
              }

              token::TokenType::Zenkaku
              | token::TokenType::Zenkigo
              | token::TokenType::Kana
              | token::TokenType::Yousoku
              | token::TokenType::Alpha
              | token::TokenType::Hankigo
              | token::TokenType::Kuten
              | token::TokenType::Special => {
                t = token::Token2 {
                  seq: (seq + 1),
                  ty: self.tokens[j].ty,
                  word: self.tokens[j].word.to_owned(),
                  ruby: None,
                };

                ruby_tokens.push(t);
                seq += 1;
                //ruby_len += self.tokens[j].word.chars().count() as i32;

                //if ruby_len > 50 {
                //  break;
                //}
              }

              _ => {
                break;
              }
            }

            j += 1;
          }
        }
      }

      if ruby_e > (ruby_s + 1) {
        // ルビあり
        t = token::Token2 {
          seq: (seq - ruby_tokens.len() as isize),
          ty: self.tokens[i].ty,
          word: self.tokens[i].word.to_owned(),
          ruby: Some(ruby_tokens),
        };

        self.token2s.push(t);
        seq += 1;
        i = ruby_e + 1;
      } else {
        // ルビなし
        match self.tokens[i].ty {
          token::TokenType::Zenkaku
          | token::TokenType::Zenkigo
          | token::TokenType::Kana
          | token::TokenType::Yousoku
          | token::TokenType::Kuten
          | token::TokenType::Space
          | token::TokenType::Special => {
            for c in self.tokens[i].word.chars() {
              if c == ' ' {
                t = token::Token2 {
                  seq: seq,
                  ty: self.tokens[i].ty,
                  word: c.to_string(),
                  ruby: None,
                };
              } else {
                t = token::Token2 {
                  seq: seq,
                  ty: self.tokens[i].ty,
                  word: c.to_string(),
                  ruby: None,
                };
              }

              self.token2s.push(t);
              seq += 1;
            }
          }

          token::TokenType::Alpha | token::TokenType::Hankigo => {
            t = token::Token2 {
              seq: seq,
              ty: self.tokens[i].ty,
              word: self.tokens[i].word.to_owned(),
              ruby: None,
            };

            self.token2s.push(t);
            seq += 1;
          }

          token::TokenType::Slash => {
            t = token::Token2 {
              seq: seq,
              ty: token::TokenType::Slash,
              word: String::from("/"),
              ruby: None,
            };

            self.token2s.push(t);
            seq += 1;
          }

          _ => {}
        }

        i += 1;
      }
    }
  }

  /*
    pub fn sprit_box_line(&mut self) {
      self.box_lines.clear();
      if self.ty == 0 {
        let mut t1 = -1;
        let mut w1 = -1;
        let mut t2 = -1;
        let mut w2 = -1;
        let mut l = 0;
        let mut i = 0;

        for t in &self.tokens {
          match t.ty {
            token::TokenType::Zenkaku
            | token::TokenType::Zenkigo
            | token::TokenType::Kana
            | token::TokenType::Yousoku
            | token::TokenType::Hankigo => {
              let mut j = 0;
              for c in t.word.chars() {
                if t1 == -1 {
                  t1 = i;
                  w1 = j;
                }
                if l > 2 {
                  let bl = BoxLine::new(t1, w1, t2, w2);
                  self.box_lines.push(bl);
                  t1 = i;
                  w1 = j;
                  l = 1;
                } else {
                  l += 1;
                }
                t2 = i;
                w2 = j;
                j += 1;
              }
            }
            token::TokenType::Alpha => {
              if t1 != -1 {
                if l > 0 {
                  let bl = BoxLine::new(t1, w1, t2, w2);
                  self.box_lines.push(bl);
                }
              }
              let bl = BoxLine::new(i, 0, i, t.word.len() as isize - 1);
              self.box_lines.push(bl);
              t1 = -1;
              l = 0;
            }
            token::TokenType::Kuten => {
              let mut j = 0;
              for c in t.word.chars() {
                if t1 == -1 {
                  t1 = i;
                  w1 = j;
                }
                if l > 3 {
                  let bl = BoxLine::new(t1, w1, t2, w2);
                  self.box_lines.push(bl);
                  t1 = i;
                  w1 = j;
                  l = 1;
                } else {
                  l += 1;
                }
                t2 = i;
                w2 = j;
                j += 1;
              }
            }
            _ => {}
          }

          i += 1;
        }

        if t1 != -1 {
          let bl = BoxLine::new(t1, w1, t2, w2);
          self.box_lines.push(bl);
        }
      }
    }
  */
  pub fn to_string(&self) -> String {
    format!(
      "Source: seq={} ty={} align={} text={}",
      self.seq, self.ty, self.align, self.text
    )
  }

  pub fn print(&self) {
    log!("{}", self.to_string());

    for t in &self.tokens {
      log!("{}", t.to_string());
    }
  }
}
