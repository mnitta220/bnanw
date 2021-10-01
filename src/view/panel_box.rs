use super::super::manager;
//use super::super::model::source;
use super::super::model::token;
use super::area;
use super::canvas;
//use super::scroll_bar;
use wasm_bindgen::prelude::*;

pub struct Content {
  pub seq: isize,
  pub ty: isize,
  pub is_dummy: bool,
  pub is_cur: bool,
  pub page: isize,
  pub index: isize,
  pub label: Option<String>,
  pub tokens: Vec<token::Token>,
  pub children: Vec<Content>,
}

impl Content {
  pub fn new(seq: isize, ty: isize, is_dummy: bool) -> Self {
    let c = Content {
      seq,
      ty,
      is_dummy,
      is_cur: false,
      page: 0,
      index: 0,
      label: None,
      tokens: Vec::new(),
      children: Vec::new(),
    };

    c
  }
}

/*
pub struct Box {
  pub ty: isize,
  pub source: isize,
  pub token: isize,
  pub word: isize,
}

impl Box {
  pub fn new(ty: isize, source: isize, token: isize, word: isize) -> Self {
    let b = Box {
      ty,
      source,
      token,
      word,
    };

    b
  }

  //pub fn draw(&mut self, cv: &canvas::Canvas, is_dark: bool) -> Result<isize, &'static str> {
  //  Ok(0)
  //}
}
*/

pub struct PanelBox {
  pub width: f64,
  pub height: f64,
  pub touching: bool,
  //pub boxs: Vec<Box>,
  pub font_size: isize,
  pub fontw1: f64,
  pub fontw2: f64,
  pub fontwh: f64,
  pub tree: Option<Content>,
  pub areas: Vec<area::Area>,
  //pub ty1: isize,
  //pub source1: isize,
  //pub token1: isize,
  //pub word1: isize,
  //pub scroll_bar: Option<scroll_bar::ScrollBar>,
}

impl PanelBox {
  pub fn new(mgr: &manager::Manager) -> Self {
    let mut pb = PanelBox {
      width: 0.0,
      height: 0.0,
      touching: false,
      //boxs: Vec::new(),
      font_size: 30,
      fontw1: 0.0,
      fontw2: 0.0,
      fontwh: 0.0,
      tree: None,
      areas: Vec::new(),
      //scroll_bar: None,
    };

    let (_, root) = pb.build_sub(mgr, 0, 0, 0, false, true);
    pb.tree = Some(root);

    if let Some(cv) = &mgr.canvas {
      //let scroll_bar = scroll_bar::ScrollBar::new(true, 1.0, cv.height - 10.0, cv.width - 2.0);
      //let scroll_bar = scroll_bar::ScrollBar::new(false, cv.width - 10.0, 1.0, cv.height - 2.0);
      //pb.scroll_bar = Some(scroll_bar);
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

    let mut con = Content::new(idx as isize, lvl, is_dummy);
    let lv = lvl + 1;
    let mut i = idx;
    //let mut is_con = false;
    //let mut has_cur = false;
    //let mut is_cur = false;

    if seq == mgr.section {
      con.is_cur = true;
    }

    loop {
      if i >= mgr.sources.len() {
        break;
      }

      let s = &mgr.sources[i];
      //log!("***PanelBox.build_sub2: s.seq={}", s.seq);

      if s.ty == 0 {
        if s.tokens.len() == 0 {
          i += 1;
          continue;
        }

        let mut c = Content::new(s.seq, s.ty, false);

        for t in &s.tokens {
          if c.label.is_none() {
            match t.ty {
              token::TokenType::Zenkaku | token::TokenType::Kana | token::TokenType::Alpha => {
                for ch in t.word.chars() {
                  let l = format!("{}", ch);
                  c.label = Some(l.clone());
                  if con.label.is_none() {
                    con.label = Some(l);
                  }
                  break;
                }
              }
              _ => {}
            }
          }
          c.tokens.push(t.clone());
        }

        if con.is_cur && con.children.len() == 0 {
          c.is_cur = true;
        }
        con.children.push(c);
        i += 1;
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
      //log!("***PanelBox.build_sub3: s.seq={}", s.seq);
      if content.is_cur {
        //is_cur = true;
        con.is_cur = true;
        //has_cur = true;
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

    /*
    log!(
      "***PanelBox.build_sub2: idx={} seq={} lvl={} mgr.section={}",
      idx,
      seq,
      lvl,
      mgr.section
    );
    log!(
      "***PanelBox.build_sub3: is_cur={} is_con={} has_cur={} con.children.len()={}",
      con.is_cur,
      is_con,
      has_cur,
      con.children.len()
    );
    */
    /*
    if con.is_cur && is_con && has_cur == false && con.children.len() > 0 {
      con.children[0].is_cur = true;
    }
    */

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
    //let font: &str;
    //font = "30pt Arial";
    let font = format!("{}pt Arial", font_size);
    cv.context.set_font(font.as_str());
    self.fontw1 = cv.context.measure_text("あ").unwrap().width();
    self.fontw2 = self.fontw1 * 1.3;
    self.fontwh = self.fontw2 / 2.0;

    if let Some(tr) = &self.tree {
      self.draw_sub(cv, tr, 1, is_dark);
    }

    // draw_base
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
    //let mut i = 0;
    let mut is_con = false;
    let mut is_sec = false;
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut l = 0;
    //for c in &con.children {
    loop {
      if i >= con.children.len() {
        break;
      }

      let c = &con.children[i];

      if c.ty > 0 {
        j += 1;
        if j > 8 {
          j = 0;
          k = i;
        }

        is_con = true;
        is_sec = false;

        if c.is_cur {
          l = k;
        }
      } else {
        if is_sec == false {
          j += 1;
          if j > 8 {
            j = 0;
            k = i;
          }
        }
        is_sec = true;
      }

      i += 1;
    }

    if is_con {
      i = l;
      j = 0;
      is_sec = false;

      loop {
        if i >= con.children.len() || j > 8 {
          break;
        }

        let c = &con.children[i];

        if c.ty > 0 {
          is_sec = false;
          if c.label.is_some() {
            log!(
              "***PanelBox.draw_sub(con): lvl={} label={} is_cur={}",
              lvl,
              c.label.clone().unwrap(),
              c.is_cur
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
          j += 1;
          if c.is_cur {
            self.draw_sub(cv, c, lvl + 1, is_dark);
          }
        } else {
          if is_sec == false {
            if c.label.is_some() {
              log!(
                "***PanelBox.draw_sub(sec): lvl={} label={} is_cur={}",
                lvl,
                c.label.clone().unwrap(),
                c.is_cur
              );
              self.draw_label(
                lvl,
                cv,
                is_dark,
                j,
                c.label.clone().unwrap().as_ref(),
                c.is_cur,
              );

              if c.is_cur {
                self.draw_text(cv, con, lvl + 1, is_dark, 0);
              }
            }
            j += 1;
          }
          is_sec = true;
        }

        i += 1;
      }
    } else {
      for c in &con.children {
        if c.label.is_some() {
          log!(
            "***PanelBox.draw_sub(sec2): lvl={} label={}",
            lvl,
            c.label.clone().unwrap()
          );
        }
      }
    }

    self.draw_frame(lvl, cv, is_dark);
  }
  /*
  pub fn set_info(&mut self, font_size: isize, cv: &canvas::Canvas) {
    self.font_size = font_size;
    //let font: &str;
    //font = "30pt Arial";
    let font = format!("{}pt Arial", font_size);
    cv.context.set_font(font.as_str());
    self.fontw1 = cv.context.measure_text("あ").unwrap().width();
    self.fontw2 = self.fontw1 * 1.3;
    self.fontwh = self.fontw2 / 2.0;
    //self.boxs.clear();
  }
  */

  /*
  pub fn draw_base(&self, cv: &canvas::Canvas, is_dark: bool) {
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
  }
  */

  fn draw_frame(&self, ty: isize, cv: &canvas::Canvas, is_dark: bool) {
    //log!("***PanelBox.draw_frame: ty={}", ty);
    //let font: &str;
    //font = "30pt Arial";
    //cv.context.set_font(font);
    //let w = cv.context.measure_text("あ").unwrap().width();
    //let w2 = w * 1.3;
    let top = self.fontw1 / 2.0 + (self.fontw2 * 4.0 * (ty - 1) as f64);
    let pad = self.fontw1 * 0.1;
    let w3 = self.fontw2 * 3.0;
    //let wh = w2 / 2.0;
    let x3 = cv.width / 2.0 - self.fontw2 / 2.0;
    let x4 = x3 + self.fontw2;
    let x2 = x3 - pad;
    let x1 = x2 - self.fontw2;
    let x5 = x4 + pad;
    //let x6 = x5 + self.fontw2;
    let y1 = top;
    let y2 = y1 + self.fontw2;
    let y3 = y2 + self.fontw2;
    //let y4 = y3 + self.fontw2;

    /*
    let mut i = 1;
    loop {
      if i >= ty {
        break;
      }
      y1 = y4 + self.fontw2;
      y2 = y1 + self.fontw2;
      y3 = y2 + self.fontw2;
      y4 = y3 + self.fontw2;
      i += 1;
    }
    */

    /*
    if ty > 1 {
      cv.context
        .fill_rect(self.fontwh, y1 - self.fontwh, cv.width - self.fontw2, 0.5);
    }
    */
    /*
    cv.context.set_line_width(0.5);
    if is_dark {
      //cv.context.set_stroke_style(&JsValue::from_str("#999999"));
      cv.context.set_fill_style(&JsValue::from_str("#999999"));
    } else {
      //.context.set_stroke_style(&JsValue::from_str("#999999"));
      cv.context.set_fill_style(&JsValue::from_str("#ffff00"));
    }
    cv.context.fill_rect(x5, y1, self.fontw2, self.fontw2);
    */

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

    /*
    cv.context.fill_rect(wh, y4 + wh, cv.width - w2, 0.5);

    if is_dark {
      cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
    } else {
      cv.context.set_fill_style(&JsValue::from_str("#000000"));
    }
    cv.context.set_text_align("center");
    cv.context.set_text_baseline("middle");
    cv.context.fill_text("夫", x5 + wh, y1 + wh).unwrap();
    cv.context.fill_text("一", x5 + wh, y2 + wh).unwrap();
    cv.context.fill_text("又", x5 + wh, y3 + wh).unwrap();
    cv.context.fill_text("此", x3 + wh, y1 + wh).unwrap();
    cv.context.fill_text("故", x3 + wh, y2 + wh).unwrap();
    cv.context.fill_text("此", x3 + wh, y3 + wh).unwrap();
    cv.context.fill_text("是", x1 + wh, y1 + wh).unwrap();
    cv.context.fill_text("二", x1 + wh, y2 + wh).unwrap();
    cv.context.fill_text("一", x1 + wh, y3 + wh).unwrap();

    let y1 = y4 + w2;
    let y2 = y1 + w2;
    let y3 = y2 + w2;
    let y4 = y3 + w2;

    if is_dark {
      //cv.context.set_stroke_style(&JsValue::from_str("#999999"));
      cv.context.set_fill_style(&JsValue::from_str("#999999"));
    } else {
      //.context.set_stroke_style(&JsValue::from_str("#999999"));
      cv.context.set_fill_style(&JsValue::from_str("#ffff00"));
    }
    cv.context.fill_rect(x5, y3, w2, w2);

    if is_dark {
      cv.context.set_stroke_style(&JsValue::from_str("#999999"));
      cv.context.set_fill_style(&JsValue::from_str("#999999"));
    } else {
      cv.context.set_stroke_style(&JsValue::from_str("#999999"));
      cv.context.set_fill_style(&JsValue::from_str("#999999"));
    }

    cv.context.stroke_rect(x1, y1, w2, w3);
    cv.context.stroke_rect(x3, y1, w2, w3);
    cv.context.stroke_rect(x5, y1, w2, w3);
    cv.context.fill_rect(x1, y2, w2, 0.5);
    cv.context.fill_rect(x1, y3, w2, 0.5);
    cv.context.fill_rect(x3, y2, w2, 0.5);
    cv.context.fill_rect(x3, y3, w2, 0.5);
    cv.context.fill_rect(x5, y2, w2, 0.5);
    cv.context.fill_rect(x5, y3, w2, 0.5);

    cv.context.fill_rect(wh, y4 + wh, cv.width - w2, 0.5);

    if is_dark {
      cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
    } else {
      cv.context.set_fill_style(&JsValue::from_str("#000000"));
    }
    cv.context.set_text_align("center");
    cv.context.set_text_baseline("middle");
    cv.context.fill_text("此", x5 + wh, y1 + wh).unwrap();
    cv.context.fill_text("性", x5 + wh, y2 + wh).unwrap();
    cv.context.fill_text("故", x5 + wh, y3 + wh).unwrap();
    cv.context.fill_text("然", x3 + wh, y1 + wh).unwrap();

    let y1 = y4 + w2;
    let y2 = y1 + w2;
    let y3 = y2 + w2;
    let y4 = y3 + w2;

    if is_dark {
      cv.context.set_stroke_style(&JsValue::from_str("#999999"));
      cv.context.set_fill_style(&JsValue::from_str("#999999"));
    } else {
      cv.context.set_stroke_style(&JsValue::from_str("#999999"));
      cv.context.set_fill_style(&JsValue::from_str("#999999"));
    }

    cv.context.stroke_rect(x1, y1, w2, w3);
    cv.context.stroke_rect(x3, y1, w2, w3);
    cv.context.stroke_rect(x5, y1, w2, w3);
    cv.context.fill_rect(x1, y2, w2, 0.5);
    cv.context.fill_rect(x1, y3, w2, 0.5);
    cv.context.fill_rect(x3, y2, w2, 0.5);
    cv.context.fill_rect(x3, y3, w2, 0.5);
    cv.context.fill_rect(x5, y2, w2, 0.5);
    cv.context.fill_rect(x5, y3, w2, 0.5);

    let font = "30pt Serif";
    cv.context.set_font(font);
    if is_dark {
      cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
    } else {
      cv.context.set_fill_style(&JsValue::from_str("#000000"));
    }
    cv.context.set_text_align("center");
    cv.context.set_text_baseline("middle");
    cv.context.fill_text("故", x5 + wh, y1 + wh).unwrap();
    cv.context.fill_text("に", x5 + wh, y2 + wh).unwrap();
    cv.context.fill_text("止", x5 + wh, y3 + wh).unwrap();
    cv.context.fill_text("観", x3 + wh, y1 + wh).unwrap();
    cv.context.fill_text("に", x3 + wh, y2 + wh).unwrap();
    cv.context.fill_text("云", x3 + wh, y3 + wh).unwrap();
    cv.context.fill_text("く", x1 + wh, y1 + wh).unwrap();
    cv.context.fill_text("「", x1 + wh, y2 + wh).unwrap();
    cv.context.fill_text("前", x1 + wh, y3 + wh).unwrap();

    cv.context.set_line_width(4.0);
    cv.context.set_line_join("round");
    cv.context.set_stroke_style(&JsValue::from_str("#0000ff"));
    cv.context.set_fill_style(&JsValue::from_str("#0000ff"));
    cv.context.move_to(100.0, 100.0);
    cv.context.line_to(140.0, 70.0);
    cv.context.line_to(140.0, 130.0);
    cv.context.line_to(100.0, 100.0);
    cv.context.stroke();
    */
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
    //let w = cv.context.measure_text("あ").unwrap().width();
    //let w2 = w * 1.3;
    let top = self.fontw1 / 2.0 + (self.fontw2 * 4.0 * (ty - 1) as f64);
    let pad = self.fontw1 * 0.1;
    //let w3 = self.fontw2 * 3.0;
    //let wh = w2 / 2.0;
    let x3 = cv.width / 2.0 - self.fontw2 / 2.0;
    let x4 = x3 + self.fontw2;
    let x2 = x3 - pad;
    let x1 = x2 - self.fontw2;
    let x5 = x4 + pad;
    //let x6 = x5 + self.fontw2;
    let y1 = top;
    let y2 = y1 + self.fontw2;
    let y3 = y2 + self.fontw2;
    let x: f64;
    let y: f64;
    match index {
      0 => {
        x = x5;
        y = y1;
      }
      1 => {
        x = x5;
        y = y2;
      }
      2 => {
        x = x5;
        y = y3;
      }
      3 => {
        x = x3;
        y = y1;
      }
      4 => {
        x = x3;
        y = y2;
      }
      5 => {
        x = x3;
        y = y3;
      }
      6 => {
        x = x1;
        y = y1;
      }
      7 => {
        x = x1;
        y = y2;
      }
      8 => {
        x = x1;
        y = y3;
      }
      _ => {
        x = x5;
        y = y1;
      }
    }

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

  fn draw_text(
    &self,
    cv: &canvas::Canvas,
    con: &Content,
    ty: isize,
    is_dark: bool,
    index: isize,
    //label: &str,
    //is_cur: bool,
  ) {
    let font = format!("{}pt Arial", self.font_size);
    cv.context.set_font(font.as_ref());
    //let w = cv.context.measure_text("あ").unwrap().width();
    //let w2 = w * 1.3;
    let top = self.fontw1 / 2.0 + (self.fontw2 * 4.0 * (ty - 1) as f64);
    let pad = self.fontw1 * 0.1;
    //let w3 = self.fontw2 * 3.0;
    //let wh = w2 / 2.0;
    let x3 = cv.width / 2.0 - self.fontw2 / 2.0;
    let x4 = x3 + self.fontw2;
    let x2 = x3 - pad;
    let x1 = x2 - self.fontw2;
    let x5 = x4 + pad;
    //let x6 = x5 + self.fontw2;
    let y1 = top;
    let y2 = y1 + self.fontw2;
    let y3 = y2 + self.fontw2;
    let mut found = false;

    for c in &con.children {
      if c.is_cur {
        found = true;
      }
      if found && c.ty != 0 {
        break;
      }
      for t in &c.tokens {
        log!("***PanelBox.draw_text: word={}", t.word);
      }
    }

    self.draw_frame(ty, cv, is_dark);
  }

  /*
  pub fn draw_sec(
    &mut self,
    cv: &canvas::Canvas,
    is_dark: bool,
    ty: isize,
    src: isize,
    token: isize,
    word: &str,
    is_cur: bool,
  ) {
    let top = self.fontw1 / 2.0;
    let pad = self.fontw1 * 0.1;
    //let w3 = self.fontw2 * 3.0;
    //let wh = w2 / 2.0;
    let x3 = cv.width / 2.0 - self.fontw2 / 2.0;
    let x4 = x3 + self.fontw2;
    let x2 = x3 - pad;
    let x1 = x2 - self.fontw2;
    let x5 = x4 + pad;
    //let x6 = x5 + self.fontw2;
    let y1 = top;
    let y2 = y1 + self.fontw2;
    let y3 = y2 + self.fontw2;
    //let y4 = y3 + self.fontw2;

    if self.boxs.len() < 9 {
      if let Some(c) = word.chars().next() {
        let b = Box::new(ty, src, token, 0);
        self.boxs.push(b);
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        match self.boxs.len() {
          1 => {
            x = x5 + self.fontwh;
            y = y1 + self.fontwh;
          }
          2 => {
            x = x5 + self.fontwh;
            y = y2 + self.fontwh;
          }
          3 => {
            x = x5 + self.fontwh;
            y = y3 + self.fontwh;
          }
          4 => {
            x = x3 + self.fontwh;
            y = y1 + self.fontwh;
          }
          5 => {
            x = x3 + self.fontwh;
            y = y2 + self.fontwh;
          }
          6 => {
            x = x3 + self.fontwh;
            y = y3 + self.fontwh;
          }
          7 => {
            x = x1 + self.fontwh;
            y = y1 + self.fontwh;
          }
          8 => {
            x = x1 + self.fontwh;
            y = y2 + self.fontwh;
          }
          9 => {
            x = x1 + self.fontwh;
            y = y3 + self.fontwh;
          }
          _ => {}
        }

        if is_cur {
          if is_dark {
            //cv.context.set_stroke_style(&JsValue::from_str("#999999"));
            cv.context.set_fill_style(&JsValue::from_str("#999999"));
          } else {
            //.context.set_stroke_style(&JsValue::from_str("#999999"));
            cv.context.set_fill_style(&JsValue::from_str("#ffff00"));
          }
          cv.context
            .fill_rect(x - self.fontwh, y - self.fontwh, self.fontw2, self.fontw2);
        }

        if is_dark {
          cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
        } else {
          cv.context.set_fill_style(&JsValue::from_str("#000000"));
        }
        cv.context.set_text_align("center");
        cv.context.set_text_baseline("middle");
        cv.context
          .fill_text(format!("{}", c).as_str(), x, y)
          .unwrap();
      }
    }
  }
  */

  /*
    pub fn draw(
      &mut self,
      cv: &canvas::Canvas,
      is_dark: bool,
      sources: &Vec<source::Source>,
    ) -> Result<isize, &'static str> {
      let mut ty: isize = 0;
      cv.clear(is_dark);

      self.draw_box(cv, is_dark);

      for s in sources {
        if s.ty == 0 {
          let mut lc = 0;
          for l in &s.box_lines {
            if lc > 2 {
              break;
            }
            //let w = &s.tokens[l.token1 as usize].word;
            log!(
              "***PanelBox.draw: ({}:{})-({}:{})",
              l.token1,
              l.word1,
              l.token2,
              l.word2
            );
            //let w = &s.tokens[l.token1 as usize].word;
            let mut tc = 0;
            let mut start = false;
            let mut end = false;
            for t in &s.tokens {
              if tc >= l.token1 {
                let mut cc = 0;
                for c in t.word.chars() {
                  if tc == l.token1 && cc == l.word1 {
                    start = true;
                  }
                  if tc == l.token2 && cc == l.word2 {
                    end = true;
                  }
                  if start {
                    log!("***PanelBox.draw: {}", c);
                  }
                  if end {
                    break;
                  }
                  cc += 1;
                }
              }
              if end {
                break;
              }
              tc += 1;
            }

            lc += 1;
          }
        } else {
          //
        }
      }
      //
      let font: &str;
      font = "30pt Arial";
      cv.context.set_font(font);
      let w = cv.context.measure_text("あ").unwrap().width();
      let w2 = w * 1.3;
      let top = w / 2.0;
      let pad = w * 0.1;
      let w3 = w2 * 3.0;
      let wh = w2 / 2.0;
      let x3 = cv.width / 2.0 - w2 / 2.0;
      let x4 = x3 + w2;
      let x2 = x3 - pad;
      let x1 = x2 - w2;
      let x5 = x4 + pad;
      let x6 = x5 + w2;
      let y1 = top;
      let y2 = y1 + w2;
      let y3 = y2 + w2;
      let y4 = y3 + w2;

      cv.context.set_line_width(0.5);
      if mgr.is_dark {
        //cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      } else {
        //.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#ffff00"));
      }
      cv.context.fill_rect(x3, y1, w2, w2);

      if mgr.is_dark {
        cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      } else {
        cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      }

      cv.context.stroke_rect(x1, y1, w2, w3);
      cv.context.stroke_rect(x3, y1, w2, w3);
      cv.context.stroke_rect(x5, y1, w2, w3);
      cv.context.fill_rect(x1, y2, w2, 0.5);
      cv.context.fill_rect(x1, y3, w2, 0.5);
      cv.context.fill_rect(x3, y2, w2, 0.5);
      cv.context.fill_rect(x3, y3, w2, 0.5);
      cv.context.fill_rect(x5, y2, w2, 0.5);
      cv.context.fill_rect(x5, y3, w2, 0.5);

      cv.context.fill_rect(wh, y4 + wh, cv.width - w2, 0.5);

      if mgr.is_dark {
        cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
      } else {
        cv.context.set_fill_style(&JsValue::from_str("#000000"));
      }
      cv.context.set_text_align("center");
      cv.context.set_text_baseline("middle");
      cv.context.fill_text("夫", x5 + wh, y1 + wh).unwrap();
      cv.context.fill_text("一", x5 + wh, y2 + wh).unwrap();
      cv.context.fill_text("又", x5 + wh, y3 + wh).unwrap();
      cv.context.fill_text("此", x3 + wh, y1 + wh).unwrap();
      cv.context.fill_text("故", x3 + wh, y2 + wh).unwrap();
      cv.context.fill_text("此", x3 + wh, y3 + wh).unwrap();
      cv.context.fill_text("是", x1 + wh, y1 + wh).unwrap();
      cv.context.fill_text("二", x1 + wh, y2 + wh).unwrap();
      cv.context.fill_text("一", x1 + wh, y3 + wh).unwrap();

      let y1 = y4 + w2;
      let y2 = y1 + w2;
      let y3 = y2 + w2;
      let y4 = y3 + w2;

      if mgr.is_dark {
        //cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      } else {
        //.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#ffff00"));
      }
      cv.context.fill_rect(x5, y3, w2, w2);

      if mgr.is_dark {
        cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      } else {
        cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      }

      cv.context.stroke_rect(x1, y1, w2, w3);
      cv.context.stroke_rect(x3, y1, w2, w3);
      cv.context.stroke_rect(x5, y1, w2, w3);
      cv.context.fill_rect(x1, y2, w2, 0.5);
      cv.context.fill_rect(x1, y3, w2, 0.5);
      cv.context.fill_rect(x3, y2, w2, 0.5);
      cv.context.fill_rect(x3, y3, w2, 0.5);
      cv.context.fill_rect(x5, y2, w2, 0.5);
      cv.context.fill_rect(x5, y3, w2, 0.5);

      cv.context.fill_rect(wh, y4 + wh, cv.width - w2, 0.5);

      if mgr.is_dark {
        cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
      } else {
        cv.context.set_fill_style(&JsValue::from_str("#000000"));
      }
      cv.context.set_text_align("center");
      cv.context.set_text_baseline("middle");
      cv.context.fill_text("此", x5 + wh, y1 + wh).unwrap();
      cv.context.fill_text("性", x5 + wh, y2 + wh).unwrap();
      cv.context.fill_text("故", x5 + wh, y3 + wh).unwrap();
      cv.context.fill_text("然", x3 + wh, y1 + wh).unwrap();

      let y1 = y4 + w2;
      let y2 = y1 + w2;
      let y3 = y2 + w2;
      let y4 = y3 + w2;

      if mgr.is_dark {
        cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      } else {
        cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      }

      cv.context.stroke_rect(x1, y1, w2, w3);
      cv.context.stroke_rect(x3, y1, w2, w3);
      cv.context.stroke_rect(x5, y1, w2, w3);
      cv.context.fill_rect(x1, y2, w2, 0.5);
      cv.context.fill_rect(x1, y3, w2, 0.5);
      cv.context.fill_rect(x3, y2, w2, 0.5);
      cv.context.fill_rect(x3, y3, w2, 0.5);
      cv.context.fill_rect(x5, y2, w2, 0.5);
      cv.context.fill_rect(x5, y3, w2, 0.5);

      let font = "30pt Serif";
      cv.context.set_font(font);
      if mgr.is_dark {
        cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
      } else {
        cv.context.set_fill_style(&JsValue::from_str("#000000"));
      }
      cv.context.set_text_align("center");
      cv.context.set_text_baseline("middle");
      cv.context.fill_text("故", x5 + wh, y1 + wh).unwrap();
      cv.context.fill_text("に", x5 + wh, y2 + wh).unwrap();
      cv.context.fill_text("止", x5 + wh, y3 + wh).unwrap();
      cv.context.fill_text("観", x3 + wh, y1 + wh).unwrap();
      cv.context.fill_text("に", x3 + wh, y2 + wh).unwrap();
      cv.context.fill_text("云", x3 + wh, y3 + wh).unwrap();
      cv.context.fill_text("く", x1 + wh, y1 + wh).unwrap();
      cv.context.fill_text("「", x1 + wh, y2 + wh).unwrap();
      cv.context.fill_text("前", x1 + wh, y3 + wh).unwrap();

      cv.context.set_line_width(4.0);
      cv.context.set_line_join("round");
      cv.context.set_stroke_style(&JsValue::from_str("#0000ff"));
      cv.context.set_fill_style(&JsValue::from_str("#0000ff"));
      cv.context.move_to(100.0, 100.0);
      cv.context.line_to(140.0, 70.0);
      cv.context.line_to(140.0, 130.0);
      cv.context.line_to(100.0, 100.0);
      cv.context.stroke();
      //

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

    fn draw_box(&mut self, cv: &canvas::Canvas, is_dark: bool) {
      let font: &str;
      font = "30pt Arial";
      cv.context.set_font(font);
      let w = cv.context.measure_text("あ").unwrap().width();
      let w2 = w * 1.3;
      let top = w / 2.0;
      let pad = w * 0.1;
      let w3 = w2 * 3.0;
      let wh = w2 / 2.0;
      let x3 = cv.width / 2.0 - w2 / 2.0;
      let x4 = x3 + w2;
      let x2 = x3 - pad;
      let x1 = x2 - w2;
      let x5 = x4 + pad;
      let x6 = x5 + w2;
      let y1 = top;
      let y2 = y1 + w2;
      let y3 = y2 + w2;
      let y4 = y3 + w2;

      cv.context.set_line_width(0.5);
      if is_dark {
        //cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      } else {
        //.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#ffff00"));
      }
      cv.context.fill_rect(x3, y1, w2, w2);

      if is_dark {
        cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      } else {
        cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      }

      cv.context.stroke_rect(x1, y1, w2, w3);
      cv.context.stroke_rect(x3, y1, w2, w3);
      cv.context.stroke_rect(x5, y1, w2, w3);
      cv.context.fill_rect(x1, y2, w2, 0.5);
      cv.context.fill_rect(x1, y3, w2, 0.5);
      cv.context.fill_rect(x3, y2, w2, 0.5);
      cv.context.fill_rect(x3, y3, w2, 0.5);
      cv.context.fill_rect(x5, y2, w2, 0.5);
      cv.context.fill_rect(x5, y3, w2, 0.5);

      cv.context.fill_rect(wh, y4 + wh, cv.width - w2, 0.5);

      if is_dark {
        cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
      } else {
        cv.context.set_fill_style(&JsValue::from_str("#000000"));
      }
      cv.context.set_text_align("center");
      cv.context.set_text_baseline("middle");
      cv.context.fill_text("夫", x5 + wh, y1 + wh).unwrap();
      cv.context.fill_text("一", x5 + wh, y2 + wh).unwrap();
      cv.context.fill_text("又", x5 + wh, y3 + wh).unwrap();
      cv.context.fill_text("此", x3 + wh, y1 + wh).unwrap();
      cv.context.fill_text("故", x3 + wh, y2 + wh).unwrap();
      cv.context.fill_text("此", x3 + wh, y3 + wh).unwrap();
      cv.context.fill_text("是", x1 + wh, y1 + wh).unwrap();
      cv.context.fill_text("二", x1 + wh, y2 + wh).unwrap();
      cv.context.fill_text("一", x1 + wh, y3 + wh).unwrap();

      let y1 = y4 + w2;
      let y2 = y1 + w2;
      let y3 = y2 + w2;
      let y4 = y3 + w2;

      if is_dark {
        //cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      } else {
        //.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#ffff00"));
      }
      cv.context.fill_rect(x5, y3, w2, w2);

      if is_dark {
        cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      } else {
        cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      }

      cv.context.stroke_rect(x1, y1, w2, w3);
      cv.context.stroke_rect(x3, y1, w2, w3);
      cv.context.stroke_rect(x5, y1, w2, w3);
      cv.context.fill_rect(x1, y2, w2, 0.5);
      cv.context.fill_rect(x1, y3, w2, 0.5);
      cv.context.fill_rect(x3, y2, w2, 0.5);
      cv.context.fill_rect(x3, y3, w2, 0.5);
      cv.context.fill_rect(x5, y2, w2, 0.5);
      cv.context.fill_rect(x5, y3, w2, 0.5);

      cv.context.fill_rect(wh, y4 + wh, cv.width - w2, 0.5);

      if is_dark {
        cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
      } else {
        cv.context.set_fill_style(&JsValue::from_str("#000000"));
      }
      cv.context.set_text_align("center");
      cv.context.set_text_baseline("middle");
      cv.context.fill_text("此", x5 + wh, y1 + wh).unwrap();
      cv.context.fill_text("性", x5 + wh, y2 + wh).unwrap();
      cv.context.fill_text("故", x5 + wh, y3 + wh).unwrap();
      cv.context.fill_text("然", x3 + wh, y1 + wh).unwrap();

      let y1 = y4 + w2;
      let y2 = y1 + w2;
      let y3 = y2 + w2;
      let y4 = y3 + w2;

      if is_dark {
        cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      } else {
        cv.context.set_stroke_style(&JsValue::from_str("#999999"));
        cv.context.set_fill_style(&JsValue::from_str("#999999"));
      }

      cv.context.stroke_rect(x1, y1, w2, w3);
      cv.context.stroke_rect(x3, y1, w2, w3);
      cv.context.stroke_rect(x5, y1, w2, w3);
      cv.context.fill_rect(x1, y2, w2, 0.5);
      cv.context.fill_rect(x1, y3, w2, 0.5);
      cv.context.fill_rect(x3, y2, w2, 0.5);
      cv.context.fill_rect(x3, y3, w2, 0.5);
      cv.context.fill_rect(x5, y2, w2, 0.5);
      cv.context.fill_rect(x5, y3, w2, 0.5);

      let font = "30pt Serif";
      cv.context.set_font(font);
      if is_dark {
        cv.context.set_fill_style(&JsValue::from_str("#ffffff"));
      } else {
        cv.context.set_fill_style(&JsValue::from_str("#000000"));
      }
      cv.context.set_text_align("center");
      cv.context.set_text_baseline("middle");
      cv.context.fill_text("故", x5 + wh, y1 + wh).unwrap();
      cv.context.fill_text("に", x5 + wh, y2 + wh).unwrap();
      cv.context.fill_text("止", x5 + wh, y3 + wh).unwrap();
      cv.context.fill_text("観", x3 + wh, y1 + wh).unwrap();
      cv.context.fill_text("に", x3 + wh, y2 + wh).unwrap();
      cv.context.fill_text("云", x3 + wh, y3 + wh).unwrap();
      cv.context.fill_text("く", x1 + wh, y1 + wh).unwrap();
      cv.context.fill_text("「", x1 + wh, y2 + wh).unwrap();
      cv.context.fill_text("前", x1 + wh, y3 + wh).unwrap();

      cv.context.set_line_width(4.0);
      cv.context.set_line_join("round");
      cv.context.set_stroke_style(&JsValue::from_str("#0000ff"));
      cv.context.set_fill_style(&JsValue::from_str("#0000ff"));
      cv.context.move_to(100.0, 100.0);
      cv.context.line_to(140.0, 70.0);
      cv.context.line_to(140.0, 130.0);
      cv.context.line_to(100.0, 100.0);
      cv.context.stroke();
    }
  */

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
