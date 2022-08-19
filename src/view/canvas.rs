use wasm_bindgen::prelude::*;

pub struct Canvas {
  pub canvas: web_sys::HtmlCanvasElement,
  pub context: web_sys::CanvasRenderingContext2d,
  pub is_vertical: bool,
  pub width: f64,
  pub height: f64,
  pub base_font: String,
  pub con_font: String,
  pub ruby_font: String,
  pub met: f64,
  pub metr: f64,
  pub metsp: f64,
  pub padding: f64,
  pub x1: f64,
  pub y1: f64,
  pub x2: f64,
  pub y2: f64,
  pub ruby_w: f64,
  pub line_margin: f64,
}

impl Canvas {
  pub fn new(
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    width: i32,
    height: i32,
    is_vertical: bool,
    font_size: isize,
    is_google: bool,
  ) -> Self {
    let w: f64 = width as f64;
    let h: f64 = height as f64;
    let f: &str;

    if is_google {
      f = "googleFont";
    } else {
      f = "Serif";
    }

    let base_font = &format!("{}pt {}", font_size, f);
    let con_font = &format!("{}pt Arial", font_size);
    let ruby_pt: f32 = font_size as f32 * 0.5;
    let mut ruby_font = format!("{}", ruby_pt);

    if let Some(n) = ruby_font.find('.') {
      ruby_font = (&ruby_font[0..n + 2]).to_string();
    }

    ruby_font = format!("{}pt {}", ruby_font, f);
    context.set_font(&ruby_font);
    let metr = context.measure_text("あ").unwrap().width();
    context.set_font(base_font);
    let met = context.measure_text("あ").unwrap().width();
    let metsp = context.measure_text(" ").unwrap().width();
    let padding: f64 = 10.0;
    let x1: f64 = padding;
    let y1: f64 = padding;
    let mut x2: f64 = w - padding;
    let mut y2: f64 = h - padding;
    let ruby_w = met / 3.0;
    //let line_margin: f64 = met * 0.39;
    let line_margin: f64; // = met * 0.39;

    if is_vertical {
      y2 -= padding;
      //let w2 = x2 - x1;
      let c = (w / (met * 1.72)) as i32;
      line_margin = (w - (met + ruby_w) * (c as f64)) / (c as f64);
    } else {
      x2 -= padding;
      let c = (h / (met * 1.72)) as i32;
      line_margin = (h - (met + ruby_w) * (c as f64)) / (c as f64);
    }

    Canvas {
      canvas: canvas,
      context: context,
      is_vertical: is_vertical,
      width: w,
      height: h,
      base_font: String::from(base_font),
      con_font: String::from(con_font),
      ruby_font: String::from(&ruby_font),
      met: met,
      metsp: metsp,
      metr: metr,
      padding: padding,
      x1: x1,
      y1: y1,
      x2: x2,
      y2: y2,
      ruby_w: ruby_w,
      line_margin: line_margin,
    }
  }

  pub fn clear(&self, is_dark: bool) {
    if is_dark {
      self.context.set_fill_style(&JsValue::from_str("#000000"));
    } else {
      self.context.set_fill_style(&JsValue::from_str("#ffffff"));
    }

    self.context.fill_rect(0.0, 0.0, self.width, self.height);
  }
}
