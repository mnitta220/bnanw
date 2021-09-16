use super::canvas;
use core::f64::consts::PI;
use wasm_bindgen::prelude::*;

pub const SCROLL_HEIGHT: i32 = 15;

pub struct ScrollBar {
  pub is_vertical: bool,
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub panel_width: f64,
  pub touching: bool,
  pub bar_touching: bool,
  pub start_x: i32,
  pub start_y: i32,
  pub cur_x: i32,
  pub cur_y: i32,
  pub start_time: f64,
  pub p1: f64,
  pub p2: f64,
}

impl ScrollBar {
  pub fn new(is_vertical: bool, x: f64, y: f64, width: f64) -> Self {
    ScrollBar {
      is_vertical: is_vertical,
      x: x,
      y: y,
      width: width,
      panel_width: 0.0,
      touching: false,
      bar_touching: false,
      start_x: 0,
      start_y: 0,
      cur_x: 0,
      cur_y: 0,
      start_time: 0.0,
      p1: 0.0,
      p2: 0.0,
    }
  }

  pub fn draw(&mut self, cv: &canvas::Canvas, pos: f64, is_dark: bool) -> Result<(), &'static str> {
    //log!("***ScrollBar.draw: panel_width={} pos={}", panel_width, pos);
    if self.is_vertical {
      if is_dark {
        cv.context.set_fill_style(&JsValue::from_str("#333333"));
      } else {
        cv.context.set_fill_style(&JsValue::from_str("#f8f8f8"));
      }

      cv.context.fill_rect(self.x, self.y + 1.0, self.width, 8.0);

      if is_dark {
        cv.context.set_fill_style(&JsValue::from_str("#888888"));
      } else {
        cv.context.set_fill_style(&JsValue::from_str("#c0c0c0"));
      }

      cv.context.fill_rect(self.x, self.y, self.width, 0.5);

      if self.panel_width > self.width {
        let w = self.panel_width + (self.width * 0.5);
        let r = self.width / w;
        self.p1 = (w - pos - self.width) * r;
        self.p2 = (w - pos) * r;

        if is_dark {
          cv.context.set_fill_style(&JsValue::from_str("#777777"));
        } else {
          cv.context.set_fill_style(&JsValue::from_str("#c0c0c0"));
        }

        cv.context.begin_path();
        cv.context
          .arc(self.p1 + 3.0, self.y + 5.0, 3.0, 0.0, PI * 2.0)
          .unwrap();
        cv.context.fill();
        cv.context.begin_path();
        cv.context
          .arc(self.p2 - 3.0, self.y + 5.0, 3.0, 0.0, PI * 2.0)
          .unwrap();
        cv.context.fill();
        cv.context
          .fill_rect(self.p1 + 3.0, self.y + 2.0, self.p2 - self.p1 - 6.0, 6.0);
      }
    } else {
      /*
      log!(
        "***ScrollBar.draw: panel_width={} width={}",
        self.panel_width,
        self.width
      );
      */
      if is_dark {
        cv.context.set_fill_style(&JsValue::from_str("#333333"));
      } else {
        cv.context.set_fill_style(&JsValue::from_str("#f8f8f8"));
      }

      cv.context.fill_rect(self.x + 1.0, self.y, 8.0, self.width);

      if is_dark {
        cv.context.set_fill_style(&JsValue::from_str("#888888"));
      } else {
        cv.context.set_fill_style(&JsValue::from_str("#c0c0c0"));
      }

      cv.context.fill_rect(self.x, self.y, 0.5, self.width);

      if self.panel_width > self.width {
        let w = self.panel_width + (self.width * 0.5);
        let r = self.width / w;
        self.p1 = -pos * r + 1.0;
        self.p2 = (-pos + self.width) * r - 1.0;

        if is_dark {
          cv.context.set_fill_style(&JsValue::from_str("#777777"));
        } else {
          cv.context.set_fill_style(&JsValue::from_str("#c0c0c0"));
        }

        cv.context.begin_path();
        cv.context
          .arc(self.x + 5.0, self.p1 + 3.0, 3.0, 0.0, PI * 2.0)
          .unwrap();
        cv.context.fill();
        cv.context.begin_path();
        cv.context
          .arc(self.x + 5.0, self.p2 - 3.0, 3.0, 0.0, PI * 2.0)
          .unwrap();
        cv.context.fill();
        cv.context
          .fill_rect(self.x + 2.0, self.p1 + 3.0, 6.0, self.p2 - self.p1 - 6.0);
      }
    }

    Ok(())
  }

  /// タッチ開始
  pub fn touch_start(&mut self, x: i32, y: i32) -> Result<(), &'static str> {
    //log!("***ScrollBar.touch_start: x={} y={}", x, y);

    if self.is_vertical {
      if self.panel_width > self.width && (self.p1 - 10.0) < x as f64 && (self.p2 + 10.0) > x as f64
      {
        self.bar_touching = true;
      } else {
        self.bar_touching = false;
      }
    } else {
      if self.panel_width > self.width && (self.p1 - 10.0) < y as f64 && (self.p2 + 10.0) > y as f64
      {
        self.bar_touching = true;
      } else {
        self.bar_touching = false;
      }
    }

    self.start_x = x;
    self.start_y = y;
    self.cur_x = x;
    self.cur_y = y;
    self.touching = true;
    self.start_time = js_sys::Date::now();

    Ok(())
  }

  /// タッチを移動する
  pub fn touch_move(&mut self, x: i32, y: i32) -> Result<isize, &'static str> {
    let mut ret: isize = -1;

    if self.bar_touching {
      self.cur_x = x;
      self.cur_y = y;
      ret = 0;
    }

    Ok(ret)
  }

  /// タッチ終了
  pub fn touch_end(&mut self) -> Result<isize, &'static str> {
    let ret: isize = 0;

    if self.bar_touching {
      self.bar_touching = false;
    }

    self.touching = false;

    Ok(ret)
  }
}
