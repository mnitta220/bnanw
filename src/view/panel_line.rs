use super::super::manager;
use super::super::model::source;
use super::super::model::token;
use super::area;
use super::canvas;
use super::panel_token;
use super::view_line;
use wasm_bindgen::prelude::*;

pub struct PanelLine {
  pub is_vertical: bool,
  pub source: isize,
  pub ty: isize,
  pub indent: f64,
  pub align: source::Align,
  pub x1: f64,
  pub y1: f64,
  pub x2: f64,
  pub y2: f64,
  pub ptokens: Vec<panel_token::PanelToken>,
  pub lines: Vec<view_line::ViewLine>,
}

impl PanelLine {
  pub fn new(is_vertical: bool, source: &source::Source, cv: &canvas::Canvas) -> Self {
    let mut indent: f64 = 0.0;
    //log!("***PanelLine.new: {}", source.to_string());

    if 0 < source.ty && source.ty < 8 {
      indent = (source.ty - 1) as f64 * 14.0;
    }

    let mut pl = PanelLine {
      is_vertical: is_vertical,
      source: source.seq as isize,
      ty: source.ty,
      indent: indent,
      align: source.align,
      x1: 0.0,
      y1: 0.0,
      x2: 0.0,
      y2: 0.0,
      ptokens: Vec::new(),
      lines: Vec::new(),
    };

    pl.analyze(source, cv);

    if is_vertical {
      pl.sep_lines(cv.y2 - cv.y1 - indent, cv);
    } else {
      pl.sep_lines(cv.x2 - cv.x1 - indent, cv);
    }

    /*
    for t in &pl.ptokens {
      log!("***{}", t.to_string());
    }

    for l in &pl.lines {
      log!("***{}", l.to_string());
    }
    */

    pl
  }

  pub fn top(is_vertical: bool) -> Self {
    let ty = manager::DOC_TOP;
    let seq = manager::DOC_TOP;

    let pl = PanelLine {
      is_vertical: is_vertical,
      source: seq,
      ty: ty,
      indent: 0.0,
      align: source::Align::Bottom,
      x1: 0.0,
      y1: 0.0,
      x2: 0.0,
      y2: 0.0,
      ptokens: Vec::new(),
      lines: Vec::new(),
    };

    pl
  }

  fn analyze(&mut self, source: &source::Source, cv: &canvas::Canvas) {
    //log!("***analyze");
    let mut i: usize = 0;
    let mut j: usize;
    let mut ruby_s: usize;
    let mut ruby_e: usize;
    let mut ruby_len: i32;
    let mut ruby_tokens: Vec<panel_token::PanelToken>;
    let mut t: panel_token::PanelToken;
    let mut seq = 0;
    let font: &str;

    if self.ty != 0 {
      font = &cv.con_font;
    } else {
      font = &cv.base_font;
    }

    cv.context.set_font(font);

    loop {
      if i >= source.tokens.len() {
        break;
      }

      ruby_s = i;
      ruby_e = i;
      ruby_len = 0;
      ruby_tokens = Vec::new();

      if (i + 1) < source.tokens.len() {
        // ルビがあるか？
        j = i + 1;

        if source.tokens[j].ty == token::TokenType::RubyS {
          ruby_s = j;
          j += 1;

          loop {
            if j >= source.tokens.len() {
              break;
            }

            match source.tokens[j].ty {
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
                cv.context.set_font(&cv.ruby_font);

                t = panel_token::PanelToken {
                  seq: (seq + 1),
                  ty: source.tokens[j].ty,
                  word: source.tokens[j].word.to_owned(),
                  ruby: None,
                  width: cv
                    .context
                    .measure_text(&source.tokens[j].word)
                    .unwrap()
                    .width(),
                };

                cv.context.set_font(font);
                ruby_tokens.push(t);
                seq += 1;
                ruby_len += source.tokens[j].word.chars().count() as i32;

                if ruby_len > 50 {
                  break;
                }
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
        t = panel_token::PanelToken {
          seq: (seq - ruby_tokens.len() as isize),
          ty: source.tokens[i].ty,
          word: source.tokens[i].word.to_owned(),
          ruby: Some(ruby_tokens),
          width: cv
            .context
            .measure_text(&source.tokens[i].word)
            .unwrap()
            .width(),
        };

        self.ptokens.push(t);
        seq += 1;
        i = ruby_e + 1;
      } else {
        // ルビなし
        match source.tokens[i].ty {
          token::TokenType::Zenkaku
          | token::TokenType::Zenkigo
          | token::TokenType::Kana
          | token::TokenType::Yousoku
          | token::TokenType::Kuten
          | token::TokenType::Space
          | token::TokenType::Special => {
            for c in source.tokens[i].word.chars() {
              if c == ' ' {
                t = panel_token::PanelToken {
                  seq: seq,
                  ty: source.tokens[i].ty,
                  word: c.to_string(),
                  ruby: None,
                  width: cv.metsp,
                };
              } else {
                t = panel_token::PanelToken {
                  seq: seq,
                  ty: source.tokens[i].ty,
                  word: c.to_string(),
                  ruby: None,
                  width: cv.met,
                };
              }

              self.ptokens.push(t);
              seq += 1;
            }
          }

          token::TokenType::Alpha | token::TokenType::Hankigo => {
            t = panel_token::PanelToken {
              seq: seq,
              ty: source.tokens[i].ty,
              word: source.tokens[i].word.to_owned(),
              ruby: None,
              width: cv
                .context
                .measure_text(&source.tokens[i].word)
                .unwrap()
                .width(),
            };

            self.ptokens.push(t);
            seq += 1;
          }

          token::TokenType::Slash => {
            t = panel_token::PanelToken {
              seq: seq,
              ty: token::TokenType::Slash,
              word: String::from("/"),
              ruby: None,
              width: 0.0,
            };

            self.ptokens.push(t);
            seq += 1;
          }

          _ => {}
        }

        i += 1;
      }
    }
  }

  fn sep_lines(&mut self, line_width: f64, cv: &canvas::Canvas) {
    log!("***PanelLine.sep_lines line_width={}", line_width);
    self.lines.clear();
    let mut tc: panel_token::PanelToken;
    let mut vl = view_line::ViewLine::new();
    let mut i: usize = 0;
    vl.align = self.align;
    for token in self.ptokens.iter() {
      log!("***token={}", token.to_string());
      i += token.word.chars().count();
      tc = panel_token::PanelToken::clone(token);
      vl.ptokens.push(tc);
      if i >= cv.char_count {
        self.lines.push(vl);
        tc = panel_token::PanelToken::clone(token);
        vl = view_line::ViewLine::new();
        vl.align = self.align;
        vl.first_token_idx = i - cv.char_count;
        i = 0;
      }
    }
    if vl.ptokens.len() > 0 {
      self.lines.push(vl);
    }
  }

  fn sep_lines_old(&mut self, line_width: f64, cv: &canvas::Canvas) {
    //log!("***PanelLine.sep_lines line_width={}", line_width);
    self.lines.clear();
    let mut tc: panel_token::PanelToken;
    let mut i: usize = 0;
    let mut br: isize;
    let mut w: f64;
    let mut w2: f64;
    let mut count: i32;
    cv.context.set_font(&cv.base_font);

    loop {
      if i >= self.ptokens.len() {
        break;
      }

      let mut vl = view_line::ViewLine::new();
      vl.align = self.align;
      w = 0.0;
      count = 0;
      br = -1;

      let mut t = &self.ptokens[i];
      w2 = t.max_width();

      if w2 > line_width {
        // 1トークンが行より長い
        let mut s1: String = String::new();
        let mut s2: String;
        let mut w3: f64;
        let mut w4: f64 = 0.0;

        for c in t.word.chars() {
          // 1文字ずつつなげて、行の長さを超える位置を調べる。
          s2 = format!("{}{}", &s1, c);
          w3 = cv.context.measure_text(&s2).unwrap().width();

          if w3 > line_width {
            tc = panel_token::PanelToken::clone(t);
            tc.word = s1;
            tc.width = w4;
            vl.ptokens.push(tc);
            vl.count = 1;
            vl.width = w4;
            self.lines.push(vl);
            vl = view_line::ViewLine::new();
            vl.align = self.align;
            s1 = format!("{}", c);
            w = 0.0;
            count = 0;
          } else {
            s1 = s2;
            w4 = w3;
          }
        }

        if s1.len() > 0 {
          tc = panel_token::PanelToken::clone(t);
          tc.word = s1;
          tc.width = w4;
          vl.ptokens.push(tc);
          w = w4;
          count = 1;
        }

        i += 1;
      } else {
        // 改行するトークンを調べる。
        let mut j = i;
        let mut w4: f64 = 0.0;

        loop {
          if j >= self.ptokens.len() {
            break;
          }

          let t2 = &self.ptokens[j];
          w2 = t2.max_width();
          w4 += w2;

          if w4 > line_width {
            br = j as isize;
            break;
          }

          j += 1;
        }

        if br != -1 {
          let mut kinsoku: bool;

          loop {
            if j == i {
              break;
            }

            let t2 = &self.ptokens[j];
            kinsoku = false;

            match t2.ty {
              token::TokenType::Kuten | token::TokenType::Yousoku => {
                kinsoku = true;
              }

              token::TokenType::Zenkigo => match &t2.word.chars().next().unwrap() {
                '」' | '』' | '）' | '】' | '］' | '｝' | '》' | '＞' => {
                  kinsoku = true;
                }

                _ => {}
              },

              _ => {}
            }

            if j > i {
              let t2 = &self.ptokens[j - 1];
              if t2.ty == token::TokenType::Zenkigo {
                match &t2.word.chars().next().unwrap() {
                  '「' | '『' | '（' | '【' | '［' | '｛' | '《' | '＜' => {
                    kinsoku = true;
                  }
                  _ => {}
                }
              }
            }

            if kinsoku == false {
              break;
            }

            j -= 1;
            br = j as isize;
          }
        }
      }

      loop {
        if i >= self.ptokens.len() {
          break;
        }

        t = &self.ptokens[i];
        w2 = t.max_width();

        match t.ty {
          token::TokenType::Zenkaku
          | token::TokenType::Zenkigo
          | token::TokenType::Alpha
          | token::TokenType::Hankigo
          | token::TokenType::Kana
          | token::TokenType::Yousoku
          | token::TokenType::Kuten
          | token::TokenType::Space
          | token::TokenType::Special => {
            count += 1;
          }

          _ => {}
        }

        if i as isize == br {
          break;
        }

        if (w + w2) > line_width {
          break;
        }

        tc = panel_token::PanelToken::clone(t);
        vl.ptokens.push(tc);
        w += w2;
        i += 1;
      }

      vl.count = count;
      vl.width = w;

      if i >= self.ptokens.len() {
        vl.last = true;
      }

      self.lines.push(vl);
    }
  }

  pub fn draw_line(
    &self,
    pos: f64,
    font_size: isize,
    cv: &canvas::Canvas,
    areas: &mut Vec<area::Area>,
    black_line: isize,
    black_token: isize,
    is_contents: bool,
    is_black: bool,
    is_gray: bool,
    is_current: bool,
    is_dark: bool,
  ) -> Result<f64, &'static str> {
    /*
    log!(
      "***draw_line: source={} align={} pos={} black_line={} black_token={} is_contents={} is_black={} is_gray={} is_current={}",
      self.source,
      self.align,
      pos,
      black_line,
      black_token,
      is_contents,
      is_black,
      is_gray,
      is_current
    );
    */

    if self.is_vertical {
      self.draw_vertical(
        pos,
        font_size,
        cv,
        areas,
        black_line,
        black_token,
        is_contents,
        is_black,
        is_gray,
        is_current,
        is_dark,
      )
    } else {
      self.draw_horizontal(
        pos,
        cv,
        areas,
        black_line,
        black_token,
        is_contents,
        is_black,
        is_gray,
        is_current,
        is_dark,
      )
    }
  }

  fn draw_vertical(
    &self,
    pos: f64,
    font_size: isize,
    cv: &canvas::Canvas,
    areas: &mut Vec<area::Area>,
    black_line: isize,
    black_token: isize,
    is_contents: bool,
    is_black: bool,
    is_gray: bool,
    is_current: bool,
    is_dark: bool,
  ) -> Result<f64, &'static str> {
    let mut x = pos + cv.met * 0.5;
    let bw = cv.met + cv.ruby_w + 1.0;
    let font: &str;
    let mut area_x1: f64 = pos;
    let area_x2: f64 = pos + bw;

    if self.ty != 0 {
      font = &cv.con_font;

      if is_dark {
        cv.context.set_stroke_style(&JsValue::from_str("#3880ff"));
        cv.context.set_fill_style(&JsValue::from_str("#3880ff"));
      } else {
        cv.context.set_stroke_style(&JsValue::from_str("#0000ff"));
        cv.context.set_fill_style(&JsValue::from_str("#0000ff"));
      }
    } else {
      font = &cv.base_font;

      if is_dark {
        cv.context.set_stroke_style(&JsValue::from_str("#ffffff"));
        cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
      } else {
        cv.context.set_stroke_style(&JsValue::from_str("#000000"));
        cv.context.set_fill_style(&JsValue::from_str("#000000"));
      }
    }
    log!("***draw_line: 1");

    cv.context.set_font(font);
    cv.context.set_text_baseline("middle");
    cv.context.set_text_align("center");

    if self.ty == manager::DOC_TOP {
      log!("***draw_line: 1.2");
      let w = cv.context.measure_text("Top").unwrap().width();
      let y = cv.y2 - w;

      if is_contents {
        if is_current {
          if is_dark {
            cv.context.set_fill_style(&JsValue::from_str("#183066"));
          } else {
            cv.context.set_fill_style(&JsValue::from_str("#dedeff"));
          }

          cv.context.fill_rect(x - 3.0, 0.0, bw, cv.height);
        }

        if is_dark {
          cv.context.set_fill_style(&JsValue::from_str("#3880ff"));
        } else {
          cv.context.set_fill_style(&JsValue::from_str("#0000ff"));
        }

        let area = area::Area::new(manager::DOC_TOP, 0, x, y, x + bw, cv.y2);
        areas.push(area);
      }

      cv.context.set_font(&cv.con_font);
      cv.context.rotate(std::f64::consts::PI / 2.0).unwrap();
      cv.context.fill_text("Top", y, -x - 2.0).unwrap();
      cv.context.rotate(-std::f64::consts::PI / 2.0).unwrap();

      x -= cv.met + cv.ruby_w + cv.line_margin;
    } else if self.ptokens.len() == 0 {
      // 空行
      x -= cv.met + cv.ruby_w + cv.line_margin;
    } else {
      log!("***draw_line: 2");
      let mut is_first = true;

      for l in &self.lines {
        area_x1 = x;

        if (-cv.met * 4.0) < x && x < (cv.canvas.width() as f64) {
          let mut y = cv.y1 + cv.char_width * 0.5; // self.indent;
          let spc: f64 = 1.0;
          let mut y1 = cv.y1;
          let mut y2: f64;
          log!(
            "***draw_line: 3 y={} indent={} char_width={} met={}",
            y,
            self.indent,
            cv.char_width,
            cv.met
          );

          if is_contents && (is_gray || is_current) {
            if is_current {
              if is_dark {
                cv.context.set_fill_style(&JsValue::from_str("#183066"));
              } else {
                cv.context.set_fill_style(&JsValue::from_str("#dedeff"));
              }
            } else if is_gray {
              if is_dark {
                cv.context.set_fill_style(&JsValue::from_str("#202020"));
              } else {
                cv.context.set_fill_style(&JsValue::from_str("#f2f2f2"));
              }
            }

            if is_first {
              cv.context.fill_rect(x, 0.0, bw, cv.height);
              is_first = false;
            } else {
              cv.context.fill_rect(x, 0.0, bw + cv.line_margin, cv.height);
            }

            if is_dark {
              cv.context.set_fill_style(&JsValue::from_str("#3880ff"));
            } else {
              cv.context.set_fill_style(&JsValue::from_str("#0000ff"));
            }
          }

          /*
          if l.last {
            log!("***draw_line: 4");
            match l.align {
              source::Align::Center | source::Align::Bottom => {
                let mut w: f64 = 0.0;

                for t in &l.ptokens {
                  w += t.max_width() + 1.0;
                }

                w = cv.y2 - cv.y1 - self.indent - w;

                if l.align == source::Align::Center {
                  y += w * 0.5;
                } else {
                  y += w;
                }

                y1 = y;
              }

              _ => {}
            }
          } else {
            log!("***draw_line: 5");
            spc = cv.y2 - cv.y1 - self.indent - l.width;

            if l.count > 2 {
              spc = spc / (l.count - 1) as f64;
            }
          }
          */

          for t in &l.ptokens {
            let mut black = false;

            if is_black {
              if self.source > black_line || (self.source == black_line && t.seq >= black_token) {
                black = true;
              }
            }

            match t.ty {
              token::TokenType::Zenkaku
              | token::TokenType::Kana
              | token::TokenType::Yousoku
              | token::TokenType::Special => {
                log!("***draw_line: 6");
                //let rw = t.ruby_width();
                //let rl = t.ruby_len();
                //let xr = x + cv.met;
                //let mut yr = y + cv.metr;
                //log!("***draw_line: rw={} t.width={}", rw, t.width);

                /*
                if rw > t.width {
                  cv.context.set_font(&cv.ruby_font);

                  if let Some(rs) = &t.ruby {
                    for r in rs {
                      for c in r.word.chars() {
                        if black == false {
                          match c {
                            '「' | '」' | '『' | '』' | '（' | '）' | '【' | '】' | '［' | '］'
                            | '｛' | '｝' | '…' | '─' | '━' | 'ー' | '＝' | '～' => {
                              cv.context.rotate(std::f64::consts::PI / 2.0).unwrap();
                              cv.context
                                .fill_text(&c.to_string(), yr - cv.metr + 2.0, -xr - 1.0)
                                .unwrap();
                              cv.context.rotate(-std::f64::consts::PI / 2.0).unwrap();
                            }

                            _ => {
                              if r.ty == token::TokenType::Yousoku {
                                cv.context
                                  .fill_text(
                                    &c.to_string(),
                                    xr + (cv.metr * 0.1),
                                    yr - (cv.metr * 0.1),
                                  )
                                  .unwrap();
                              } else {
                                cv.context.fill_text(&c.to_string(), xr, yr).unwrap();
                              }
                            }
                          }
                        }

                        yr += cv.metr;
                      }
                    }
                  }

                  cv.context.set_font(font);

                  {
                    let w = rw - t.width;
                    let l = t.word.chars().count();
                    let w = w / (l + 1) as f64;
                    let mut y3 = y + cv.met + (w * 0.8);

                    for c in t.word.chars() {
                      if black == false {
                        match t.ty {
                          token::TokenType::Special => {
                            let st = &c.to_string();
                            let w3 = cv.context.measure_text(st).unwrap().width();
                            let w3 = (cv.met - w3) * 0.5;
                            cv.context.fill_text(st, x + w3, y3).unwrap();
                          }

                          token::TokenType::Yousoku => {
                            cv.context
                              .fill_text(&c.to_string(), x + (cv.met * 0.1), y3 - (cv.met * 0.1))
                              .unwrap();
                          }

                          _ => {
                            cv.context.fill_text(&c.to_string(), x, y3).unwrap();
                          }
                        }
                      }

                      y3 += cv.met + w;
                    }
                  }

                  y = yr - cv.metr + spc;
                } else {
                }
                */

                //log!("***draw_line: 7");

                if let Some(rs) = &t.ruby {
                  let w = t.word.chars().count() as f64 * cv.char_width;
                  let rw = w / t.ruby_len() as f64;
                  let xr = x + (cv.met + cv.metr) * 0.5;
                  let mut yr = y - (cv.char_width - rw) * 0.5;
                  let ruby = cv.ruby_font_size_from_width(rw);
                  log!(
                    "***draw_line: 8 y={} t.word.len={} ruby_len={} rw={} yr={} ruby={}",
                    y,
                    t.word.chars().count(),
                    t.ruby_len(),
                    rw,
                    yr,
                    ruby
                  );
                  log!("***draw_line: 8.1 x={} y={}", x, y);
                  if ruby > 0 {
                    cv.context.set_font(&format!("{}{}", ruby, cv.ruby_part));
                    for r in rs {
                      for c in r.word.chars() {
                        if black == false {
                          match c {
                            '「' | '」' | '『' | '』' | '（' | '）' | '【' | '】' | '［' | '］'
                            | '｛' | '｝' | '…' | '─' | '━' | 'ー' | '＝' | '～' => {
                              cv.context.rotate(std::f64::consts::PI / 2.0).unwrap();
                              cv.context
                                .fill_text(&c.to_string(), yr - cv.metr + 2.0, -xr - 1.0)
                                .unwrap();
                              cv.context.rotate(-std::f64::consts::PI / 2.0).unwrap();
                            }

                            _ => {
                              if r.ty == token::TokenType::Yousoku {
                                cv.context
                                  .fill_text(
                                    &c.to_string(),
                                    xr + (cv.metr * 0.1),
                                    yr - (cv.metr * 0.1),
                                  )
                                  .unwrap();
                              } else {
                                cv.context.fill_text(&c.to_string(), xr, yr).unwrap();
                              }
                            }
                          }
                        }
                        yr += rw;
                      }
                    }
                  }
                  log!("***draw_line: 8.2 x={} y={}", x, y);

                  /*
                  cv.context.set_font(&cv.ruby_font);
                  let mut w = t.width - rw;

                  w = w / rl as f64;
                  yr += w * 0.5;

                  for r in rs {
                    for c in r.word.chars() {
                      if black == false {
                        match c {
                          '「' | '」' | '『' | '』' | '（' | '）' | '【' | '】' | '［' | '］'
                          | '｛' | '｝' | '…' | '─' | '━' | 'ー' | '＝' | '～' => {
                            cv.context.rotate(std::f64::consts::PI / 2.0).unwrap();
                            cv.context
                              .fill_text(&c.to_string(), yr - cv.metr + 2.0, -xr - 1.0)
                              .unwrap();
                            cv.context.rotate(-std::f64::consts::PI / 2.0).unwrap();
                          }

                          _ => {
                            if r.ty == token::TokenType::Yousoku {
                              cv.context
                                .fill_text(
                                  &c.to_string(),
                                  xr + (cv.metr * 0.1),
                                  yr - (cv.metr * 0.1),
                                )
                                .unwrap();
                            } else {
                              cv.context.fill_text(&c.to_string(), xr, yr).unwrap();
                            }
                          }
                        }
                      }

                      yr += cv.metr + w;
                    }
                  }
                  */
                }

                cv.context.set_font(font);
                log!("***draw_line: 8.3 x={} y={}", x, y);

                for c in t.word.chars() {
                  log!("***draw_line: 9 x={} y={}", x, y);
                  //y += cv.met;
                  //log!("***draw_line: 10 y={}", y);

                  if black == false {
                    match t.ty {
                      token::TokenType::Special => {
                        let st = &c.to_string();
                        let w3 = cv.context.measure_text(st).unwrap().width();
                        let w3 = (cv.met - w3) * 0.5;
                        cv.context.fill_text(st, x + w3, y).unwrap();
                      }

                      token::TokenType::Yousoku => {
                        cv.context
                          .fill_text(&c.to_string(), x + (cv.met * 0.1), y - (cv.met * 0.1))
                          .unwrap();
                      }

                      _ => {
                        log!("***draw_line: 11 y={}", y);
                        cv.context.fill_text(&c.to_string(), x, y).unwrap();
                      }
                    }
                  }
                  y += cv.char_width;
                }

                //y += spc;
              }

              token::TokenType::Zenkigo => {
                if black == false {
                  cv.context.rotate(std::f64::consts::PI / 2.0).unwrap();
                  //cv.context.fill_text(&t.word, y + 3.0, -x - 2.0).unwrap();
                  cv.context
                    .fill_text(&t.word, y + (cv.metr * 0.22), -x - (cv.metr * 0.22))
                    .unwrap();
                  cv.context.rotate(-std::f64::consts::PI / 2.0).unwrap();
                }

                y += t.width + spc;
              }

              token::TokenType::Alpha => {
                if black == false {
                  cv.context.rotate(std::f64::consts::PI / 2.0).unwrap();
                  cv.context.fill_text(&t.word, y + 3.0, -x - 4.0).unwrap();
                  cv.context.rotate(-std::f64::consts::PI / 2.0).unwrap();
                }

                y += t.width + spc;
              }

              token::TokenType::Hankigo => {
                if black == false {
                  cv.context.rotate(std::f64::consts::PI / 2.0).unwrap();
                  //cv.context.fill_text(&t.word, y + 3.0, -x - 2.0).unwrap();
                  cv.context
                    .fill_text(&t.word, y + 3.0, -x - (font_size as f64 * 0.3) - 1.0)
                    .unwrap();
                  cv.context.rotate(-std::f64::consts::PI / 2.0).unwrap();
                }

                y += t.width + spc;
              }

              token::TokenType::Space => {
                y += t.width + spc;
              }

              token::TokenType::Kuten => {
                if black == false {
                  cv.context
                    .fill_text(&t.word, x + (cv.met * 0.6), y + (cv.met * 0.4))
                    .unwrap();
                }

                y += t.width + spc;
              }
              _ => {}
            }

            y2 = y - y1 + 1.0;

            if y1 + y2 > cv.y2 {
              y2 = cv.y2 - y1;

              if y2 < 0.0 {
                y2 = 0.0;
              }
            }

            if t.ty != token::TokenType::Slash && t.ty != token::TokenType::Tatebo {
              if black {
                cv.context.set_fill_style(&JsValue::from_str("#555555"));
                cv.context.fill_rect(x, y1 + 1.0, bw, y2);

                if is_dark {
                  cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
                } else {
                  cv.context.set_fill_style(&JsValue::from_str("#000000"));
                }
              }

              if is_contents == false {
                let area = area::Area::new(self.source, t.seq, x, y1, x + bw, y1 + y2);
                areas.push(area);
              }
            }

            y1 = y;
          }
        }

        x -= cv.met + cv.ruby_w + cv.line_margin;
      }

      if is_contents {
        let area = area::Area::new(self.source, 0, area_x1, cv.y1, area_x2, cv.y2);
        areas.push(area);
      }
    }

    Ok(x)
  }

  fn draw_horizontal(
    &self,
    pos: f64,
    cv: &canvas::Canvas,
    areas: &mut Vec<area::Area>,
    black_line: isize,
    black_token: isize,
    is_contents: bool,
    is_black: bool,
    is_gray: bool,
    is_current: bool,
    is_dark: bool,
  ) -> Result<f64, &'static str> {
    let mut y = pos;
    let bw = cv.met + cv.ruby_w + 1.0;
    let font: &str;
    let area_y1: f64 = pos - bw;
    let mut area_y2: f64 = pos;

    if self.ty != 0 {
      font = &cv.con_font;

      if is_dark {
        cv.context.set_stroke_style(&JsValue::from_str("#3880ff"));
        cv.context.set_fill_style(&JsValue::from_str("#3880ff"));
      } else {
        cv.context.set_stroke_style(&JsValue::from_str("#0000ff"));
        cv.context.set_fill_style(&JsValue::from_str("#0000ff"));
      }
    } else {
      font = &cv.base_font;

      if is_dark {
        cv.context.set_stroke_style(&JsValue::from_str("#ffffff"));
        cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
      } else {
        cv.context.set_stroke_style(&JsValue::from_str("#000000"));
        cv.context.set_fill_style(&JsValue::from_str("#000000"));
      }
    }

    cv.context.set_font(font);

    if self.ty == manager::DOC_TOP {
      let w = cv.context.measure_text("Top").unwrap().width();
      let x = cv.x2 - w - 15.0;

      if is_contents {
        if is_current {
          if is_dark {
            cv.context.set_fill_style(&JsValue::from_str("#183066"));
          } else {
            cv.context.set_fill_style(&JsValue::from_str("#dedeff"));
          }

          cv.context.fill_rect(0.0, y - bw + 5.0, cv.width, bw + 2.0);
        }

        if is_dark {
          cv.context.set_fill_style(&JsValue::from_str("#3880ff"));
        } else {
          cv.context.set_fill_style(&JsValue::from_str("#0000ff"));
        }

        let area = area::Area::new(manager::DOC_TOP, 0, 0.0, y - bw + 5.0, cv.x2, bw + 2.0);
        areas.push(area);
      }

      cv.context.set_font(&cv.con_font);
      cv.context.fill_text("Top", x, y).unwrap();
      y += cv.met + cv.ruby_w + cv.line_margin;
    } else if self.ptokens.len() == 0 {
      // 空行
      y += cv.met + cv.ruby_w + cv.line_margin;
    } else {
      let mut is_first = true;

      for l in &self.lines {
        area_y2 = y;

        if 0.0 < y && y < (cv.canvas.height() as f64 + (cv.met * 4.0)) {
          let mut x = cv.x1 + self.indent;
          let mut spc: f64 = 1.0;
          let mut x1 = cv.x1;
          let mut x2: f64;

          if is_contents && (is_gray || is_current) {
            if is_current {
              if is_dark {
                cv.context.set_fill_style(&JsValue::from_str("#183066"));
              } else {
                cv.context.set_fill_style(&JsValue::from_str("#dedeff"));
              }
            } else if is_gray {
              if is_dark {
                cv.context.set_fill_style(&JsValue::from_str("#202020"));
              } else {
                cv.context.set_fill_style(&JsValue::from_str("#f2f2f2"));
              }
            }

            if is_first {
              cv.context.fill_rect(0.0, y - bw + 5.0, cv.width, bw + 2.0);
              is_first = false;
            } else {
              cv.context.fill_rect(
                0.0,
                y - bw - cv.line_margin + 5.0,
                cv.width,
                bw + cv.line_margin + 2.0,
              );
            }

            if is_dark {
              cv.context.set_fill_style(&JsValue::from_str("#3880ff"));
            } else {
              cv.context.set_fill_style(&JsValue::from_str("#0000ff"));
            }
          }

          if l.last {
            match l.align {
              source::Align::Center | source::Align::Bottom => {
                let mut w: f64 = 0.0;

                for t in &l.ptokens {
                  w += t.max_width() + 1.0;
                }

                w = cv.x2 - cv.x1 - self.indent - w;

                if l.align == source::Align::Center {
                  x += w * 0.5;
                } else {
                  x += w;
                }

                x1 = x;
              }

              _ => {}
            }
          } else {
            spc = cv.x2 - cv.x1 - self.indent - l.width;

            if l.count > 2 {
              spc = spc / (l.count - 1) as f64;
            }
          }

          for t in &l.ptokens {
            let mut black = false;

            if is_black {
              if self.source > black_line || (self.source == black_line && t.seq >= black_token) {
                black = true;
              }
            }

            match t.ty {
              token::TokenType::Zenkaku
              | token::TokenType::Kana
              | token::TokenType::Yousoku
              | token::TokenType::Special => {
                let rw = t.ruby_width();
                let rl = t.ruby_len();
                let yr = y - cv.met;
                let mut xr = x;

                if rw > t.width {
                  cv.context.set_font(&cv.ruby_font);

                  if let Some(rs) = &t.ruby {
                    for r in rs {
                      for c in r.word.chars() {
                        if black == false {
                          cv.context.fill_text(&c.to_string(), xr, yr).unwrap();
                        }

                        xr += cv.metr;
                      }
                    }
                  }

                  cv.context.set_font(font);

                  {
                    let w = rw - t.width;
                    let l = t.word.chars().count();
                    let w = w / (l + 1) as f64;
                    let mut x3 = x + (w * 0.8);

                    for c in t.word.chars() {
                      if black == false {
                        cv.context.fill_text(&c.to_string(), x3, y).unwrap();
                      }

                      x3 += cv.met + w;
                    }
                  }

                  x = xr + spc;
                } else {
                  cv.context.set_font(&cv.ruby_font);

                  if let Some(rs) = &t.ruby {
                    let mut w = t.width - rw;

                    w = w / rl as f64;
                    xr += w * 0.5;

                    for r in rs {
                      for c in r.word.chars() {
                        if black == false {
                          cv.context.fill_text(&c.to_string(), xr, yr).unwrap();
                        }

                        xr += cv.metr + w;
                      }
                    }
                  }

                  cv.context.set_font(font);

                  for c in t.word.chars() {
                    if black == false {
                      cv.context.fill_text(&c.to_string(), x, y).unwrap();
                    }
                    x += cv.met;
                  }

                  x += spc;
                }
              }

              token::TokenType::Zenkigo => {
                if black == false {
                  cv.context.fill_text(&t.word, x, y).unwrap();
                }

                x += t.width + spc;
              }

              token::TokenType::Alpha | token::TokenType::Hankigo => {
                if black == false {
                  cv.context.fill_text(&t.word, x, y).unwrap();
                }

                x += t.width + spc;
              }

              token::TokenType::Space => {
                x += t.width + spc;
              }

              token::TokenType::Kuten => {
                if black == false {
                  cv.context.fill_text(&t.word, x, y).unwrap();
                }

                x += t.width + spc;
              }
              _ => {}
            }

            x2 = x - x1 + 1.0;

            if x1 + x2 > cv.x2 {
              x2 = cv.x2 - x1;

              if x2 < 0.0 {
                x2 = 0.0;
              }
            }

            if t.ty != token::TokenType::Slash && t.ty != token::TokenType::Tatebo {
              if black {
                cv.context.set_fill_style(&JsValue::from_str("#555555"));
                cv.context.fill_rect(x1 + 1.0, y - bw + 5.0, x2, bw);

                if is_dark {
                  cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
                } else {
                  cv.context.set_fill_style(&JsValue::from_str("#000000"));
                }
              }

              if is_contents == false {
                let area = area::Area::new(self.source, t.seq, x1, y - bw + 5.0, x1 + x2, y + 2.0);
                areas.push(area);
              }
            }

            x1 = x;
          }
        }

        y += cv.met + cv.ruby_w + cv.line_margin;
      }

      if is_contents {
        let area = area::Area::new(self.source, 0, 0.0, area_y1, cv.width, area_y2);
        areas.push(area);
      }
    }

    Ok(y)
  }

  pub fn to_string(&self) -> String {
    format!("PanelLine: source={} ty={}", self.source, self.ty)
  }

  pub fn print(&self) {
    log!("{}", self.to_string());
    for t in &self.ptokens {
      log!("{}", t.to_string());
    }
  }
}
