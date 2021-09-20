use super::super::manager;
//use super::super::model::stroke;
use super::canvas;
use wasm_bindgen::prelude::*;

pub struct PanelBox {
  pub width: f64,
  pub height: f64,
  pub touching: bool,
  //pub strokes: Vec<stroke::Stroke>,
}

impl PanelBox {
  pub fn new(mgr: &manager::Manager) -> Self {
    let mut pb = PanelBox {
      width: 0.0,
      height: 0.0,
      touching: false,
      //strokes: Vec::new(),
    };

    if let Some(cv) = &mgr.canvas {
      pb.width = cv.width;
      pb.height = cv.height;
    }

    pb
  }

  pub fn draw(&mut self, cv: &canvas::Canvas, is_dark: bool) -> Result<isize, &'static str> {
    let font: &str;
    cv.clear(is_dark);
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
    /*
    let txt = "あいうえおかきくけこ";
    let mut i = 0;
    let mut x = 100.0; // : isize;
    let mut y = w * 1.5; //: isize;
    let pad = w * 1.4;
    for c in txt.chars() {
      if i > 8 {
        break;
      }
      x = 100.0;
      y = w * 1.3;
      match i {
        0 => {
          x += pad * 2.0;
        }
        1 => {
          x += pad * 2.0;
          y += pad;
        }
        2 => {
          x += pad * 2.0;
          y += pad * 2.0;
        }
        3 => {
          x += pad;
        }
        4 => {
          x += pad;
          y += pad;
        }
        5 => {
          x += pad;
          y += pad * 2.0;
        }
        7 => {
          y += pad;
        }
        8 => {
          y += pad * 2.0;
        }
        _ => {}
      }
      cv.context.fill_text(&c.to_string(), x, y).unwrap();
      i += 1;
    }
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
