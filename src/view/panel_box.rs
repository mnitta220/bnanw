use super::super::manager;
use super::super::model::source;
use super::canvas;
use super::scroll_bar;
use wasm_bindgen::prelude::*;

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

pub struct PanelBox {
  pub width: f64,
  pub height: f64,
  pub touching: bool,
  pub boxs: Vec<Box>,
  pub font_size: isize,
  pub fontw1: f64,
  pub fontw2: f64,
  pub fontwh: f64,
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
      boxs: Vec::new(),
      font_size: 30,
      fontw1: 0.0,
      fontw2: 0.0,
      fontwh: 0.0,
      //scroll_bar: None,
    };

    if let Some(cv) = &mgr.canvas {
      //let scroll_bar = scroll_bar::ScrollBar::new(true, 1.0, cv.height - 10.0, cv.width - 2.0);
      //let scroll_bar = scroll_bar::ScrollBar::new(false, cv.width - 10.0, 1.0, cv.height - 2.0);
      //pb.scroll_bar = Some(scroll_bar);
      pb.width = cv.width;
      pb.height = cv.height;
    }

    pb
  }

  pub fn set_info(&mut self, font_size: isize, cv: &canvas::Canvas) {
    self.font_size = font_size;
    //let font: &str;
    //font = "30pt Arial";
    let font = format!("{}pt Arial", font_size);
    cv.context.set_font(font.as_str());
    self.fontw1 = cv.context.measure_text("あ").unwrap().width();
    self.fontw2 = self.fontw1 * 1.3;
    self.fontwh = self.fontw2 / 2.0;
    self.boxs.clear();
  }

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

  pub fn draw_frame(&self, ty: isize, cv: &canvas::Canvas, is_dark: bool) {
    //let font: &str;
    //font = "30pt Arial";
    //cv.context.set_font(font);
    //let w = cv.context.measure_text("あ").unwrap().width();
    //let w2 = w * 1.3;
    let top = self.fontw1 / 2.0;
    let pad = self.fontw1 * 0.1;
    let w3 = self.fontw2 * 3.0;
    //let wh = w2 / 2.0;
    let x3 = cv.width / 2.0 - self.fontw2 / 2.0;
    let x4 = x3 + self.fontw2;
    let x2 = x3 - pad;
    let x1 = x2 - self.fontw2;
    let x5 = x4 + pad;
    //let x6 = x5 + self.fontw2;
    let mut y1 = top;
    let mut y2 = y1 + self.fontw2;
    let mut y3 = y2 + self.fontw2;
    let mut y4 = y3 + self.fontw2;

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

    if ty > 1 {
      cv.context
        .fill_rect(self.fontwh, y1 - self.fontwh, cv.width - self.fontw2, 0.5);
    }
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

  pub fn draw(
    &mut self,
    cv: &canvas::Canvas,
    is_dark: bool,
    sources: &Vec<source::Source>,
  ) -> Result<isize, &'static str> {
    let mut ty: isize = 0;
    cv.clear(is_dark);
    /*
    if let Some(sb) = &mut self.scroll_bar {
      sb.width = cv.height - 2.0;
      match sb.draw(&cv, 0.0, is_dark) {
        Err(e) => return Err(e),

        _ => {}
      }
    }
    */

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
    /*
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
    */

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
