use super::super::model::stroke;
use super::canvas;
use wasm_bindgen::prelude::*;

pub struct PanelBoard {
  pub width: f64,
  pub height: f64,
  pub touching: bool,
  pub strokes: Vec<stroke::Stroke>,
}

impl PanelBoard {
  pub fn new() -> Self {
    let pb = PanelBoard {
      width: 0.0,
      height: 0.0,
      touching: false,
      strokes: Vec::new(),
    };

    pb
  }

  pub fn set_manager(&mut self, canvas: &Option<canvas::Canvas>) {
    if let Some(cv) = &canvas {
      self.width = cv.width;
      self.height = cv.height;
    }
  }

  pub fn draw(&mut self, cv: &canvas::Canvas, is_dark: bool) -> Result<isize, &'static str> {
    cv.clear(is_dark);
    cv.context.set_line_width(3.0);
    if is_dark {
      cv.context.set_stroke_style(&JsValue::from_str("#ffffff"));
    } else {
      cv.context.set_stroke_style(&JsValue::from_str("#000000"));
    }

    for s in &self.strokes {
      cv.context.begin_path();
      let mut first: bool = true;
      for p in &s.points {
        if first {
          first = false;
          cv.context.move_to(p.x.into(), p.y.into());
        } else {
          cv.context.line_to(p.x.into(), p.y.into());
        }
      }
      cv.context.stroke();
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
    let point = stroke::Point::new(x, y);
    let mut stroke = stroke::Stroke::new();
    stroke.points.push(point);
    self.strokes.push(stroke);

    Ok(())
  }

  /// タッチを移動する
  pub fn touch_move(&mut self, x: i32, y: i32) -> Result<isize, &'static str> {
    if self.touching {
      //log!("***PanelBoard.touch_move: x={} y={}", x, y);
      if let Some(s) = self.strokes.last_mut() {
        if let Some(p) = s.points.last() {
          if p.x == x && p.y == y {
            return Ok(0);
          }
        }
        let point = stroke::Point::new(x, y);
        s.points.push(point);
      }
    }

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
    self.strokes.pop();

    Ok(0)
  }

  /// 白板・消去
  pub fn stroke_clear(&mut self) -> Result<isize, &'static str> {
    self.strokes.clear();

    Ok(0)
  }
}
