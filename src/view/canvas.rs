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
  pub x3: f64,
  pub y3: f64,
  //pub ruby_w: f64,
  pub line_margin: f64,
  pub char_width: f64,
  pub char_count: usize,
  pub ruby_pt: i32,
  pub ruby_part: String,
  pub line_width: f64,
  pub page_lines: i32,
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
    let ruby_pt: i32 = (font_size as f32 * 0.5) as i32;
    let ruby_part = format!("pt {}", f);
    let ruby_font = format!("{}{}", ruby_pt, ruby_part);
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
    //let ruby_w = met / 3.0;
    let line_margin: f64;
    let char_count: usize;
    let char_width: f64;
    let page_lines: i32;
    let x3 = x2 - met * 0.5;
    let y3 = y2 - met * 0.5;

    if is_vertical {
      y2 -= padding;
      //let c = (w / (met * 1.72)) as i32;
      page_lines = (x2 / (met * 1.2 + metr)) as i32;
      //line_margin = (x2 - 20.0) / c as f64 - met * 1.75;
      line_margin = (x2 - (met * 1.2 + metr) * (page_lines as f64)) / (page_lines as f64);
      char_count = ((y3 - y1) / met) as usize;
      char_width = (y3 - y1) / char_count as f64;
    } else {
      x2 -= padding;
      page_lines = (h / (met * 1.72)) as i32;
      line_margin = (h - (met * 1.72) * (page_lines as f64)) / (page_lines as f64);
      char_count = ((x3 - x1) / met) as usize;
      char_width = (x3 - x1) / char_count as f64;
    }
    //log! {"ruby_font: {}, ruby_pt={}, ruby_part={}", ruby_font, ruby_pt, ruby_part};

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
      x3: x3,
      y3: y3,
      //ruby_w: ruby_w,
      line_margin: line_margin,
      char_width: char_width,
      char_count: char_count,
      ruby_pt,
      ruby_part,
      line_width: met * 1.2 + metr + line_margin,
      page_lines,
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

  pub fn ruby_font_size_from_width(&self, width: f64) -> (i32, f64) {
    let mut font_pt = self.ruby_pt;
    while font_pt > 5 {
      let ruby_font = format!("{}{}", font_pt, self.ruby_part);
      self.context.set_font(&ruby_font);
      let metr = self.context.measure_text("あ").unwrap().width();
      if metr <= width {
        return (font_pt, metr);
      }
      font_pt = (font_pt as f64 * 0.9) as i32;
    }
    (0, 0.0)
  }

  pub fn draw_block(&self, x: f64, is_dark: bool) {
    self.context.set_line_width(1.0);
    if is_dark {
      self.context.set_stroke_style(&JsValue::from_str("#333333"));
    } else {
      self.context.set_stroke_style(&JsValue::from_str("#d4d4d4"));
    }
    //let mut x = diff + cv.width - cv.metr - cv.line_margin * 0.1;
    //x = x % cv.line_width;
    //log!("***x={}", x);
    let x1 = x - self.met * 0.1;
    let x2 = x + self.met * 1.1;
    let mut y: f64;
    self.context.begin_path();
    self.context.move_to(x1, self.y1);
    self.context.line_to(x1, self.y3);
    self.context.stroke();
    //x -= cv.met * 1.2;
    self.context.begin_path();
    self.context.move_to(x2, self.y1);
    self.context.line_to(x2, self.y3);
    self.context.stroke();
    y = self.y1;
    for _i in 0..=self.char_count {
      self.context.begin_path();
      self.context.move_to(x1, y);
      self.context.line_to(x2, y);
      self.context.stroke();
      y += self.char_width;
    }
  }
}
