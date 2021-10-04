use super::super::manager;
use super::super::model::contents;
use super::super::model::token;
use super::super::FuncType;
use super::area;
use super::canvas;
use wasm_bindgen::prelude::*;

pub struct PanelBox {
  pub is_vertical: bool,
  pub width: f64,
  pub height: f64,
  pub touching: bool,
  pub font_size: isize,
  pub fontw1: f64,
  pub fontw2: f64,
  pub fontwh: f64,
  pub sec_page: isize,
  pub areas: Vec<area::Area>,
}

impl PanelBox {
  pub fn new(mgr: &manager::Manager) -> Self {
    let mut pb = PanelBox {
      is_vertical: mgr.is_vertical,
      width: 0.0,
      height: 0.0,
      touching: false,
      font_size: 30,
      fontw1: 0.0,
      fontw2: 0.0,
      fontwh: 0.0,
      sec_page: 0,
      areas: Vec::new(),
    };

    if let Some(cv) = &mgr.canvas {
      pb.width = cv.width;
      pb.height = cv.height;
    }

    pb
  }

  pub fn draw(
    &mut self,
    tree: &mut contents::ContentTree,
    cv: &canvas::Canvas,
    areas: &mut Vec<area::Area>,
    is_black: bool,
    is_dark: bool,
    font_size: isize,
  ) -> Result<isize, &'static str> {
    //log!("***PanelBox.draw");

    cv.clear(is_dark);
    self.font_size = font_size;
    let font = format!("{}pt Arial", font_size);
    cv.context.set_font(font.as_str());
    self.fontw1 = cv.context.measure_text("あ").unwrap().width();
    self.fontw2 = self.fontw1 * 1.3;
    self.fontwh = self.fontw2 / 2.0;

    self.draw_sub(cv, tree, 1, is_dark);

    if is_dark {
      cv.context.set_stroke_style(&JsValue::from_str("#ffffff"));
    } else {
      cv.context.set_stroke_style(&JsValue::from_str("#000000"));
    }

    if is_dark {
      cv.context.set_fill_style(&JsValue::from_str("#888888"));
    } else {
      cv.context.set_fill_style(&JsValue::from_str("#777777"));
    }

    cv.context.fill_rect(0.0, 0.0, cv.width, 1.0);
    cv.context.fill_rect(0.0, 0.0, 1.0, cv.height);
    cv.context.fill_rect(cv.width - 1.0, 0.0, 1.0, cv.height);
    cv.context.fill_rect(0.0, cv.height - 1.0, cv.width, 1.0);

    Ok(0)
  }

  fn draw_sub(
    &self,
    cv: &canvas::Canvas,
    con: &mut contents::ContentTree,
    lvl: isize,
    is_dark: bool,
  ) {
    //log!("***PanelBox.draw_sub.0");
    let mut i = 0;
    let mut j = 0;
    let mut pg = 0;
    let mut has_con = false;
    let mut has_cur = false;

    for c in &con.children {
      if c.ty == contents::ContentType::Content {
        has_con = true;
      }
      if c.is_cur {
        has_cur = true;
      }
    }
    if (con.ty == contents::ContentType::Content || con.ty == contents::ContentType::Section)
      && has_cur == false
    {
      for c in &mut con.children {
        c.is_cur = true;
        break;
      }
    }

    loop {
      if i >= con.children.len() {
        break;
      }

      let c = &mut con.children[i];

      match c.ty {
        contents::ContentType::Content => {
          if c.page == pg {
            if c.label.is_some() {
              /*
              log!(
                "***PanelBox.draw_sub.2(con): lvl={} label={} is_cur={} pg={} j={}",
                lvl,
                c.label.clone().unwrap(),
                c.is_cur,
                pg,
                j
              );
              */
              self.draw_label(
                lvl,
                cv,
                is_dark,
                j,
                c.label.clone().unwrap().as_ref(),
                c.is_cur,
              );
            }
            if c.is_cur {
              self.draw_sub(cv, c, lvl + 1, is_dark);
            }
          }
        }
        contents::ContentType::Section => {
          if c.page == pg {
            if has_con {
              if c.label.is_some() {
                /*
                log!(
                  "***PanelBox.draw_sub.6(sec): lvl={} label={} is_cur={} pg={} j={}",
                  lvl,
                  c.label.clone().unwrap(),
                  c.is_cur,
                  pg,
                  j
                );
                */
                self.draw_label(
                  lvl,
                  cv,
                  is_dark,
                  j,
                  c.label.clone().unwrap().as_ref(),
                  c.is_cur,
                );
              }
              if c.is_cur {
                self.draw_section(cv, c, lvl + 1, is_dark);
              }
            } else {
              if c.is_cur {
                self.draw_section(cv, c, lvl, is_dark);
              }
            }
          }
        }
        contents::ContentType::Text => {}
      }

      i += 1;
      j += 1;
      if j > 8 {
        pg += 1;
        j = 0;
      }
    }

    self.draw_frame(lvl, cv, is_dark);
  }

  fn draw_frame(&self, ty: isize, cv: &canvas::Canvas, is_dark: bool) {
    //log!("***PanelBox.draw_frame: ty={}", ty);
    let (x1, x3, x5, y1, y2, y3, _) = self.pos_xy(ty, cv);
    let w3 = self.fontw2 * 3.0;

    if is_dark {
      cv.context.set_stroke_style(&JsValue::from_str("#999999"));
      cv.context.set_fill_style(&JsValue::from_str("#999999"));
    } else {
      cv.context.set_stroke_style(&JsValue::from_str("#999999"));
      cv.context.set_fill_style(&JsValue::from_str("#999999"));
    }

    cv.context.set_line_width(1.0);
    cv.context.stroke_rect(x1, y1, self.fontw2, w3);
    cv.context.stroke_rect(x3, y1, self.fontw2, w3);
    cv.context.stroke_rect(x5, y1, self.fontw2, w3);
    cv.context.fill_rect(x1, y2, self.fontw2, 0.5);
    cv.context.fill_rect(x1, y3, self.fontw2, 0.5);
    cv.context.fill_rect(x3, y2, self.fontw2, 0.5);
    cv.context.fill_rect(x3, y3, self.fontw2, 0.5);
    cv.context.fill_rect(x5, y2, self.fontw2, 0.5);
    cv.context.fill_rect(x5, y3, self.fontw2, 0.5);
  }

  fn draw_label(
    &self,
    ty: isize,
    cv: &canvas::Canvas,
    is_dark: bool,
    index: isize,
    label: &str,
    is_cur: bool,
  ) {
    let font = format!("{}pt Arial", self.font_size);
    cv.context.set_font(font.as_ref());
    let (x1, x3, x5, y1, y2, y3, _) = self.pos_xy(ty, cv);
    let (x, y) = self.idx_to_xy(index, x1, x3, x5, y1, y2, y3);

    if is_cur {
      if is_dark {
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      } else {
        cv.context.set_fill_style(&JsValue::from_str("#ffff00"));
      }
      cv.context.fill_rect(x, y, self.fontw2, self.fontw2);
    }

    if is_dark {
      cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
    } else {
      cv.context.set_fill_style(&JsValue::from_str("#000000"));
    }
    cv.context.set_text_align("center");
    cv.context.set_text_baseline("middle");
    cv.context
      .fill_text(label, x + self.fontwh, y + self.fontwh)
      .unwrap();
  }

  fn draw_section(
    &self,
    cv: &canvas::Canvas,
    con: &contents::ContentTree,
    ty: isize,
    is_dark: bool,
  ) {
    log!(
      "***PanelBox.draw_section: ty={} self.sec_page={} len={}",
      ty,
      self.sec_page,
      &con.children.len()
    );
    let font = format!("{}pt Serif", self.font_size);
    cv.context.set_font(font.as_ref());
    let (x1, x3, x5, y1, y2, y3, y4) = self.pos_xy(ty, cv);

    if is_dark {
      cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
    } else {
      cv.context.set_fill_style(&JsValue::from_str("#000000"));
    }
    cv.context.set_text_align("center");
    cv.context.set_text_baseline("middle");

    let mut i = 0;
    let mut l = 0;
    let mut pg = 0;

    for c in &con.children {
      for t in &c.token2s {
        match t.ty {
          token::TokenType::Zenkaku
          | token::TokenType::Kana
          | token::TokenType::Yousoku
          | token::TokenType::Zenkigo
          | token::TokenType::Special => {
            for ch in t.word.chars() {
              if i > 2 {
                i = 0;
                l += 1;
                if l > 2 {
                  l = 0;
                  pg += 1;
                }
              }

              if self.sec_page == pg {
                let (x, y) = self.li_to_xy(l, i, x1, x3, x5, y1, y2, y3, y4);

                if t.ty == token::TokenType::Zenkigo {
                  cv.context.rotate(std::f64::consts::PI / 2.0).unwrap();
                  cv.context
                    .fill_text(
                      &ch.to_string(),
                      y + 3.0 + self.fontwh,
                      -x - self.fontwh - 2.0,
                    )
                    .unwrap();
                  cv.context.rotate(-std::f64::consts::PI / 2.0).unwrap();
                } else {
                  cv.context
                    .fill_text(&ch.to_string(), x + self.fontwh, y + self.fontwh)
                    .unwrap();
                }
              }

              i += 1;
            }
          }
          token::TokenType::Kuten => {
            for ch in t.word.chars() {
              if i > 3 {
                i = 0;
                l += 1;
                if l > 2 {
                  l = 0;
                  pg += 1;
                }
              }

              if self.sec_page == pg {
                let (mut x, mut y) = self.li_to_xy(l, i, x1, x3, x5, y1, y2, y3, y4);

                x += cv.met * 0.9;
                y -= cv.met * 0.9;

                cv.context
                  .fill_text(&ch.to_string(), x + self.fontwh, y + self.fontwh)
                  .unwrap();
              }

              i += 1;
            }
          }
          token::TokenType::Alpha | token::TokenType::Hankigo => {
            i = 0;
            l += 1;
            if l > 2 {
              l = 0;
              pg += 1;
            }
            if self.sec_page == pg {
              let (x, y) = self.li_to_xy(l, i, x1, x3, x5, y1, y2, y3, y4);
              cv.context.rotate(std::f64::consts::PI / 2.0).unwrap();
              cv.context
                .fill_text(&t.word, y + 3.0 + self.fontwh, -x - self.fontwh - 2.0)
                .unwrap();
              cv.context.rotate(-std::f64::consts::PI / 2.0).unwrap();
            }
            l += 1;
            if l > 2 {
              l = 0;
              pg += 1;
            }
          }
          _ => {}
        }
      }

      if i > 0 {
        i = 0;
        l += 1;
        if l > 2 {
          l = 0;
          pg += 1;
        }
      }
    }

    self.draw_frame(ty, cv, is_dark);
  }

  fn pos_xy(&self, ty: isize, cv: &canvas::Canvas) -> (f64, f64, f64, f64, f64, f64, f64) {
    let top = self.fontw1 / 2.0 + (self.fontw2 * 4.0 * (ty - 1) as f64);
    let pad = self.fontw1 * 0.1;
    let x3 = cv.width / 2.0 - self.fontw2 / 2.0;
    let x4 = x3 + self.fontw2;
    let x2 = x3 - pad;
    let x1 = x2 - self.fontw2;
    let x5 = x4 + pad;
    let y1 = top;
    let y2 = y1 + self.fontw2;
    let y3 = y2 + self.fontw2;
    let y4 = y3 + self.fontw2;

    (x1, x3, x5, y1, y2, y3, y4)
  }

  fn idx_to_xy(
    &self,
    idx: isize,
    x1: f64,
    x3: f64,
    x5: f64,
    y1: f64,
    y2: f64,
    y3: f64,
  ) -> (f64, f64) {
    match idx {
      0 => (x5, y1),
      1 => (x5, y2),
      2 => (x5, y3),
      3 => (x3, y1),
      4 => (x3, y2),
      5 => (x3, y3),
      6 => (x1, y1),
      7 => (x1, y2),
      _ => (x1, y3),
    }
  }

  fn li_to_xy(
    &self,
    l: isize,
    i: isize,
    x1: f64,
    x3: f64,
    x5: f64,
    y1: f64,
    y2: f64,
    y3: f64,
    y4: f64,
  ) -> (f64, f64) {
    match l {
      0 => match i {
        0 => (x5, y1),
        1 => (x5, y2),
        2 => (x5, y3),
        _ => (x5, y4),
      },
      1 => match i {
        0 => (x3, y1),
        1 => (x3, y2),
        2 => (x3, y3),
        _ => (x3, y4),
      },
      _ => match i {
        0 => (x1, y1),
        1 => (x1, y2),
        2 => (x1, y3),
        _ => (x1, y4),
      },
    }
  }

  pub fn tool_func(&mut self, mt: FuncType) -> Result<isize, &'static str> {
    match mt {
      // 1区切り進む
      FuncType::FdSlash => {
        self.sec_page += 1;
      }
      // 1区切り戻る
      FuncType::BkSlash => {
        if self.sec_page > 0 {
          self.sec_page -= 1;
        }
      }
      // 先頭に戻る
      FuncType::BkTop => {
        self.sec_page = 0;
      }
      _ => {}
    }

    Ok(0)
  }

  /// タッチ開始
  pub fn touch_start(&mut self, x: i32, y: i32) -> Result<(), &'static str> {
    //log!("***PanelBoard.touch_start: x={} y={}", x, y);
    self.touching = true;
    /*
    let point = stroke::Point::new(x, y);
    let mut stroke = stroke::Stroke::new();
    stroke.points.push(point);
    self.strokes.push(stroke);
    */

    Ok(())
  }

  /// タッチを移動する
  pub fn touch_move(&mut self, x: i32, y: i32) -> Result<isize, &'static str> {
    Ok(0)
  }

  /// タッチ終了
  ///
  /// # 戻り値
  /// - -3 : 正常終了
  /// - -2 : ダブルタップ
  /// - -1 : Top選択
  /// - 0以上 : セクション選択
  /// - それ以外 : 異常終了
  ///
  pub fn touch_end(&mut self) -> Result<isize, &'static str> {
    //log!("***PanelBoard.touch_end");
    self.touching = false;
    Ok(0)
  }
}
