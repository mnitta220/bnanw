use super::super::manager;
use super::super::model::token;
use super::super::FuncType;
use super::area;
use super::canvas;
use wasm_bindgen::prelude::*;

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub enum ContentType {
  Content,
  Section,
  Text,
}

pub struct Content {
  pub ty: ContentType,
  pub seq: isize,
  pub lbl: isize,
  pub is_dummy: bool,
  pub is_cur: bool,
  pub page: isize,
  pub label: Option<String>,
  pub token2s: Vec<token::Token2>,
  pub children: Vec<Content>,
}

impl Content {
  pub fn new(ty: ContentType, seq: isize, lbl: isize, is_dummy: bool) -> Self {
    let c = Content {
      ty,
      seq,
      lbl,
      is_dummy,
      is_cur: false,
      page: 0,
      label: None,
      token2s: Vec::new(),
      children: Vec::new(),
    };

    c
  }

  pub fn get_cur_sec(&self) -> Option<&Content> {
    for c in &self.children {
      match c.ty {
        ContentType::Content => {
          if c.is_cur {
            return c.get_cur_sec();
          }
        }
        ContentType::Section => {
          if c.is_cur {
            return Some(c);
          }
        }
        ContentType::Text => {}
      }
    }

    None
  }
}

pub struct PanelBox {
  pub is_vertical: bool,
  pub width: f64,
  pub height: f64,
  pub touching: bool,
  pub font_size: isize,
  pub fontw1: f64,
  pub fontw2: f64,
  pub fontwh: f64,
  pub tree: Option<Content>,
  pub pages: Vec<usize>,
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
      tree: None,
      pages: Vec::new(),
      areas: Vec::new(),
    };

    pb.pages.push(0);
    pb.pages.push(0);
    pb.pages.push(0);
    pb.pages.push(0);
    pb.pages.push(0);
    pb.pages.push(0);
    pb.pages.push(0);

    let (_, root) = pb.build_sub(mgr, 0, 0, 0, false, true);
    pb.tree = Some(root);

    if let Some(cv) = &mgr.canvas {
      pb.width = cv.width;
      pb.height = cv.height;
    }

    pb
  }

  fn build_sub(
    &mut self,
    mgr: &manager::Manager,
    idx: usize,
    seq: isize,
    lvl: isize,
    is_cur: bool,
    is_dummy: bool,
  ) -> (usize, Content) {
    log!(
      "***PanelBox.build_sub: idx={} seq={} lvl={} is_cur={} mgr.section={}",
      idx,
      seq,
      lvl,
      is_cur,
      mgr.section
    );

    let mut con = Content::new(ContentType::Content, idx as isize, lvl, is_dummy);
    let lv = lvl + 1;
    let mut i = idx;
    //let mut is_con = true;

    if mgr.section == -1 {
      if seq == 0 {
        con.is_cur = true;
      }
    } else if seq == mgr.section {
      con.is_cur = true;
    }

    loop {
      if i >= mgr.sources.len() {
        break;
      }

      let s = &mgr.sources[i];

      if s.ty == 0 {
        let mut section = Content::new(ContentType::Section, s.seq, lv, false);
        //let mut j = i;
        loop {
          if i >= mgr.sources.len() {
            break;
          }
          let s2 = &mgr.sources[i];
          if s2.ty != 0 {
            break;
          }
          let mut c = Content::new(ContentType::Text, s2.seq, lv + 1, false);

          for t in &s2.token2s {
            if c.label.is_none() {
              match t.ty {
                token::TokenType::Zenkaku | token::TokenType::Kana | token::TokenType::Alpha => {
                  for ch in t.word.chars() {
                    let l = format!("{}", ch);
                    c.label = Some(l.clone());
                    if section.label.is_none() {
                      section.label = Some(l);
                    }
                    break;
                  }
                }
                _ => {}
              }
            }
            c.token2s.push(t.clone());
          }
          section.children.push(c);
          i += 1;
        }

        if con.label.is_none() && section.label.is_some() {
          con.label = Some(section.label.clone().unwrap());
        }
        if con.is_cur && con.children.len() == 0 {
          section.is_cur = true;
        }
        con.children.push(section);
        continue;
      }

      if s.ty < lv {
        break;
      }

      //is_con = true;
      let dm: bool;
      if s.ty == lv {
        i += 1;
        dm = false;
      } else {
        dm = true;
      }

      let (index, mut content) = self.build_sub(mgr, i, s.seq, lv, con.is_cur, dm);

      i = index;
      if content.is_cur {
        con.is_cur = true;
      }

      if con.label.is_none() && content.label.is_some() {
        let l = content.label.clone().unwrap();
        con.label = Some(l);
      }

      if is_cur && con.children.len() == 0 {
        content.is_cur = true;
      }

      con.children.push(content);
    }

    (i, con)
  }

  pub fn draw(
    &mut self,
    cv: &canvas::Canvas,
    areas: &mut Vec<area::Area>,
    is_black: bool,
    is_dark: bool,
    font_size: isize,
  ) -> Result<isize, &'static str> {
    log!("***PanelBox.draw");

    cv.clear(is_dark);
    self.font_size = font_size;
    let font = format!("{}pt Arial", font_size);
    cv.context.set_font(font.as_str());
    self.fontw1 = cv.context.measure_text("あ").unwrap().width();
    self.fontw2 = self.fontw1 * 1.3;
    self.fontwh = self.fontw2 / 2.0;

    if let Some(tr) = &self.tree {
      self.draw_sub(cv, tr, 1, is_dark);
    }

    cv.context.set_line_width(3.0);
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

  fn draw_sub(&self, cv: &canvas::Canvas, con: &Content, lvl: isize, is_dark: bool) {
    let mut i = 0;
    let mut j = 0;
    let mut pg = 0;

    loop {
      if i >= con.children.len() {
        break;
      }

      let c = &con.children[i];

      match c.ty {
        ContentType::Content => {
          if con.page == pg {
            if c.label.is_some() {
              log!(
                "***PanelBox.draw_sub(con): lvl={} label={} is_cur={} pg={} j={}",
                lvl,
                c.label.clone().unwrap(),
                c.is_cur,
                pg,
                j
              );
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
        ContentType::Section => {
          if con.page == pg {
            if c.label.is_some() {
              log!(
                "***PanelBox.draw_sub(sec): lvl={} label={} is_cur={} pg={} j={}",
                lvl,
                c.label.clone().unwrap(),
                c.is_cur,
                pg,
                j
              );
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
          }
        }
        ContentType::Text => {
          //
        }
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

  fn draw_section(&self, cv: &canvas::Canvas, con: &Content, ty: isize, is_dark: bool) {
    log!("***PanelBox.draw_section: ty={}", ty);
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

              if con.page == pg {
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

              if con.page == pg {
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
            if con.page == pg {
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
        self.scroll(mt);
      }
      _ => {}
    }

    Ok(0)
  }

  fn scroll(&mut self, mt: FuncType) {
    if let Some(tr) = &mut self.tree {
      //let mut cur1 = false;
      tr.page = 1;
      if let Some(sec) = &mut tr.get_cur_sec() {
        //sec.page = 1;
      }
      /*
      for c1 in &tr.children {
        match c1.ty {
          ContentType::Content => {
            //
          }
          ContentType::Section => {
            //
          }
          ContentType::Text => {}
        }
      }
      */
    }
  }

  fn scroll_old(&mut self, mt: FuncType) {
    if let Some(tr) = &self.tree {
      let mut cur1 = false;
      for c1 in &tr.children {
        if c1.lbl == 0 {
          continue;
        }
        if c1.is_cur {
          cur1 = true;
          let mut cur2 = false;
          for c2 in &c1.children {
            if c2.lbl == 0 {
              continue;
            }
            if c2.is_cur {
              cur2 = true;
              let mut cur3 = false;
              for c3 in &c2.children {
                if c3.lbl == 0 {
                  continue;
                }
                if c3.is_cur {
                  cur3 = true;
                  let mut cur4 = false;
                  for c4 in &c3.children {
                    if c4.lbl == 0 {
                      continue;
                    }
                    if c4.is_cur {
                      cur4 = true;
                      let mut cur5 = false;
                      for c5 in &c1.children {
                        if c5.lbl == 0 {
                          continue;
                        }
                        if c5.is_cur {
                          cur5 = true;
                          break;
                        }
                      }
                      if cur5 == false {
                        //
                      }
                      break;
                    }
                  }
                  if cur4 == false {
                    //
                  }
                  break;
                }
              }
              if cur3 == false {
                //
              }
              break;
            }
          }
          if cur2 == false {
            //
          }
          break;
        }
      }
      if cur1 == false {
        //
      }
    }
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

  /// 白板・戻る
  pub fn stroke_back(&mut self) -> Result<isize, &'static str> {
    //self.strokes.pop();

    Ok(0)
  }

  /// 白板・消去
  pub fn stroke_clear(&mut self) -> Result<isize, &'static str> {
    //self.strokes.clear();

    Ok(0)
  }
}
