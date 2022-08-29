use super::super::manager;
use super::super::model::source;
use super::super::model::token;
use super::super::FuncType;
use super::area;
use super::canvas;
use super::panel;
use super::panel_line;
use super::scroll_bar;
use wasm_bindgen::prelude::*;

pub struct PanelContents {
  pub is_vertical: bool,
  pub font_size: isize,
  pub pos: f64,
  pub touching: bool,
  pub start_x: i32,
  pub start_y: i32,
  pub cur_x: i32,
  pub cur_y: i32,
  pub start_time: f64,
  pub width: f64,
  pub height: f64,
  pub panel_width: f64,
  pub current: isize,
  pub black_source: isize,
  pub black_token: isize,
  pub scroll_bar: Option<scroll_bar::ScrollBar>,
  pub plines: Vec<panel_line::PanelLine>,
  pub areas: Vec<area::Area>,
  pub is_black: bool,
}

impl panel::Panel for PanelContents {
  fn new() -> Self {
    //log!("***PanelContents.new");

    let pc = PanelContents {
      is_vertical: false,
      font_size: 0,
      pos: 0.0,
      touching: false,
      start_x: 0,
      start_y: 0,
      cur_x: 0,
      cur_y: 0,
      start_time: 0.0,
      width: 0.0,
      height: 0.0,
      panel_width: 0.0,
      current: manager::DOC_TOP,
      black_source: -1,
      black_token: 0,
      scroll_bar: None,
      plines: Vec::new(),
      areas: Vec::new(),
      is_black: false,
    };

    pc
  }

  fn draw(
    &mut self,
    cv: &canvas::Canvas,
    areas: &mut Vec<area::Area>,
    is_black: bool,
    is_dark: bool,
    is_hide: bool,
    is_hide_block: bool,
  ) -> Result<isize, &'static str> {
    /*log!(
      "***PanelContents.draw: current={} is_hide={}",
      self.current,
      is_hide
    );
    */
    cv.clear(is_dark);
    self.is_black = is_black;

    if let Some(sb) = &mut self.scroll_bar {
      let mut diff: f64 = 0.0;

      if self.is_vertical {
        let mut x: f64;

        if sb.panel_width > sb.width {
          x = self.pos + cv.x2 - cv.met * 1.1 - cv.metr;

          if sb.bar_touching {
            diff = (sb.start_x - sb.cur_x) as f64 * sb.panel_width / sb.width;
          } else if self.touching {
            diff = (self.cur_x - self.start_x) as f64;
          }

          x += diff;
          let p = sb.panel_width - self.pos;
          if sb.width * 0.6 > p {
            self.pos = sb.panel_width - sb.width * 0.6;
            x = self.pos + cv.x2 - cv.met * 1.1 - cv.metr;
          }
        } else {
          x = sb.width - cv.met * 1.1 - cv.metr;
          self.pos = 0.0;
        }

        if is_hide == false || is_black {
          let is_gray = false;

          for l in &self.plines {
            match l.draw_line(
              x,
              self.font_size,
              cv,
              areas,
              self.black_source,
              self.black_token,
              true,
              is_black,
              is_gray,
              l.source == self.current,
              is_dark,
              is_hide_block,
            ) {
              Ok(r) => x = r,

              Err(e) => {
                return Err(e);
              }
            }
          }

          //is_gray = !is_gray;
        }
      } else {
        let mut y: f64;

        if sb.panel_width > sb.width {
          y = self.pos + cv.met * 1.1 + cv.metr;

          if sb.bar_touching {
            diff = (sb.start_y - sb.cur_y) as f64 * sb.panel_width / sb.width;
          } else if self.touching {
            diff = (self.cur_y - self.start_y) as f64;
          }

          y += diff;
          let p = sb.panel_width + self.pos;
          if sb.width * 0.6 > p {
            self.pos = sb.width * 0.6 - sb.panel_width;
            y = self.pos + cv.met * 1.1 + cv.metr;
          }
        } else {
          y = cv.met * 1.1 + cv.metr;
          self.pos = 0.0;
        }

        if is_hide == false || is_black {
          let is_gray = false;

          for l in &self.plines {
            match l.draw_line(
              y,
              self.font_size,
              cv,
              areas,
              self.black_source,
              self.black_token,
              true,
              is_black,
              is_gray,
              l.source == self.current,
              is_dark,
              is_hide_block,
            ) {
              Ok(r) => y = r,

              Err(e) => {
                return Err(e);
              }
            }
          }

          //is_gray = !is_gray;
        }
      }

      match sb.draw(&cv, self.pos + diff, is_dark) {
        Err(e) => return Err(e),

        _ => {}
      }
    }

    cv.context.set_fill_style(&JsValue::from_str("#3880ff"));
    cv.context.fill_rect(0.0, 0.0, self.width, 2.0);
    cv.context.fill_rect(0.0, 0.0, 2.0, self.height);
    cv.context
      .fill_rect(self.width - 2.0, 0.0, self.width, self.height);
    cv.context
      .fill_rect(0.0, self.height - 2.0, self.width, self.height);

    Ok(0)
  }

  /// タッチ開始
  fn touch_start(&mut self, x: i32, y: i32) -> Result<(), &'static str> {
    //log!("***PanelContents.touch_start: x={} y={}", x, y);
    self.start_x = x;
    self.start_y = y;
    self.cur_x = x;
    self.cur_y = y;

    match &mut self.scroll_bar {
      Some(sb) => {
        if self.is_vertical {
          if sb.bar_touching {
            self.pos += (sb.start_x - sb.cur_x) as f64 * sb.panel_width / sb.width;
          } else if self.touching {
            self.pos += (self.cur_x - self.start_x) as f64;
          }

          if y > (self.height as i32 - scroll_bar::SCROLL_HEIGHT) {
            sb.touch_start(x, y)
          } else {
            self.touching = true;
            self.start_time = js_sys::Date::now();

            Ok(())
          }
        } else {
          if sb.bar_touching {
            self.pos += (sb.start_y - sb.cur_y) as f64 * sb.panel_width / sb.width;
          } else if self.touching {
            self.pos += (self.cur_y - self.start_y) as f64;
          }

          if x > (self.width as i32 - scroll_bar::SCROLL_HEIGHT) {
            sb.touch_start(x, y)
          } else {
            self.touching = true;
            self.start_time = js_sys::Date::now();

            Ok(())
          }
        }
      }

      _ => Err("PanelContents.touch_start scroll_bar None"),
    }
  }

  /// タッチを移動する
  fn touch_move(&mut self, x: i32, y: i32) -> Result<isize, &'static str> {
    //log!("***PanelContents.touch_move: x={} y={}", x, y);
    let mut ret: isize = -1;

    if self.is_vertical {
      if y > (self.height as i32 - scroll_bar::SCROLL_HEIGHT) {
        match &mut self.scroll_bar {
          Some(sb) => {
            if sb.bar_touching {
              let p1 = sb.p1 - 3.0 + (x - sb.cur_x) as f64;

              if p1 > 0.0 {
                let p2 = sb.p2 + 3.0 + (x - sb.cur_x) as f64;

                if p2 < sb.width {
                  if let Err(e) = sb.touch_move(x, y) {
                    return Err(e);
                  }

                  self.cur_x = x;
                  self.cur_y = y;
                  ret = 0;
                }
              }
            }
          }

          _ => return Err("PanelContents.touch_move scroll_bar None"),
        }
      } else {
        if self.touching {
          let p1 = self.pos + (x - self.start_x) as f64;

          if let Some(a) = self.areas.last() {
            let p2 = a.x2 + (x - self.start_x) as f64;

            if p2 < (self.width * 0.5) && p1 > 0.0 {
              self.cur_x = x;
              self.cur_y = y;
              ret = 0;
            }
          }
        }
      }
    } else {
      if x > (self.width as i32 - scroll_bar::SCROLL_HEIGHT) {
        match &mut self.scroll_bar {
          Some(sb) => {
            if sb.bar_touching {
              let p1 = sb.p1 - 3.0 + (y - sb.cur_y) as f64;

              if p1 > 0.0 {
                let p2 = sb.p2 + 3.0 + (y - sb.cur_y) as f64;

                if p2 < sb.width {
                  if let Err(e) = sb.touch_move(x, y) {
                    return Err(e);
                  }

                  self.cur_x = x;
                  self.cur_y = y;
                  ret = 0;
                }
              }
            }
          }

          _ => return Err("PanelContents.touch_move scroll_bar None"),
        }
      } else {
        if self.touching {
          let p1 = self.pos + (y - self.start_y) as f64;

          if let Some(a) = self.areas.last() {
            let p2 = a.y2 + (y - self.start_y) as f64;

            if p2 > (self.height * 0.5) && p1 <= 0.0 {
              self.cur_x = x;
              self.cur_y = y;
              ret = 0;
            }
          }
        }
      }
    }

    Ok(ret)
  }

  /// タッチ終了
  ///
  /// # 戻り値
  /// - -3 : 正常終了
  /// - -2 : ダブルタップ
  /// - -1 : Top選択
  /// - 0以上 : セクション選択
  /// - -4 : ドラッグ終了
  /// - それ以外 : 異常終了
  ///
  fn touch_end(&mut self) -> Result<isize, &'static str> {
    //log!("***PanelContents.touch_end");
    let ret: isize = -3;
    let mut is_sb: bool = false;

    match &mut self.scroll_bar {
      Some(sb) => {
        if self.is_vertical {
          if self.cur_y > (self.height as i32 - scroll_bar::SCROLL_HEIGHT) {
            is_sb = true;

            if sb.bar_touching {
              self.pos += (sb.start_x - sb.cur_x) as f64 * sb.panel_width / sb.width;
            }

            if let Err(e) = sb.touch_end() {
              return Err(e);
            }
          }
        } else {
          if self.cur_x > (self.width as i32 - scroll_bar::SCROLL_HEIGHT) {
            is_sb = true;

            if sb.bar_touching {
              self.pos += (sb.start_y - sb.cur_y) as f64 * sb.panel_width / sb.width;
            }

            if let Err(e) = sb.touch_end() {
              return Err(e);
            }
          }
        }

        sb.bar_touching = false;
      }

      _ => return Err("PanelSection.touch_end scroll_bar None"),
    }

    if is_sb == false {
      let diff3;

      if self.is_vertical {
        diff3 = self.cur_x - self.start_x;
      } else {
        diff3 = self.cur_y - self.start_y;
      }

      if self.touching {
        self.pos += diff3 as f64;
      }
    }

    self.touching = false;

    Ok(ret)
  }
}

impl PanelContents {
  pub fn set_manager(
    &mut self,
    is_vertical: bool,
    font_size: isize,
    canvas: &Option<canvas::Canvas>,
    contents: &Vec<usize>,
    sources: &Vec<source::Source>,
  ) {
    self.is_vertical = is_vertical;
    self.font_size = font_size;

    if let Some(cv) = &canvas {
      let scroll_bar;

      if is_vertical {
        scroll_bar = scroll_bar::ScrollBar::new(true, 2.0, cv.height - 11.0, cv.width - 4.0);
      } else {
        scroll_bar = scroll_bar::ScrollBar::new(false, cv.width - 11.0, 2.0, cv.height - 4.0);
      }

      self.scroll_bar = Some(scroll_bar);
      self.plines.clear();
      let pl = panel_line::PanelLine::top(is_vertical);
      self.plines.push(pl);

      for c in contents {
        //log!("***PanelContents PanelLine");
        let pl = panel_line::PanelLine::new(is_vertical, &sources[*c], &cv);
        self.plines.push(pl);
      }

      self.width = cv.width;
      self.height = cv.height;
      let panel_width = (cv.met * 1.2 + cv.metr + cv.line_margin) * (self.count_lines() + 1) as f64;
      self.set_panel_width(panel_width);
    }
  }

  /// 行数カウント
  fn count_lines(&self) -> usize {
    let mut c: usize = 0;

    for l in &self.plines {
      c += l.lines.len();
    }

    c
  }

  /// パネル幅設定
  fn set_panel_width(&mut self, panel_width: f64) {
    self.panel_width = panel_width;

    if let Some(s) = &mut self.scroll_bar {
      s.panel_width = panel_width;
    }
  }

  /// シングルクリック
  ///
  /// # 戻り値
  /// - -3 : 正常終了
  /// - -2 : ダブルタップ
  /// - -1 : Top選択
  /// - 0以上 : セクション選択
  /// - それ以外 : 異常終了
  ///
  pub fn single_click(&mut self, x: i32, y: i32) -> Result<isize, &'static str> {
    //log!("***PanelContents.single_click: x={} y={}", x, y);
    self.cur_x = x;
    self.cur_y = y;
    let ret: isize;
    let (section, _) = area::Area::touch_pos(&self.areas, x as f64, y as f64);
    if self.is_black && self.black_source <= section {
      ret = -3;
    } else {
      ret = section;
    }
    Ok(ret)
  }

  /// ダブルクリック
  pub fn double_click(&mut self, x: i32, y: i32) -> Result<isize, &'static str> {
    //log!("***PanelContents.double_click: x={} y={}", x, y);
    self.cur_x = x;
    self.cur_y = y;
    let (source, token) = area::Area::touch_pos(&self.areas, self.cur_x as f64, self.cur_y as f64);

    if source >= 0 {
      self.black_source = source;
      self.black_token = token;
    }

    Ok(0)
  }

  /// 黒塗りを移動する
  pub fn tool_func(&mut self, mt: FuncType, cv: &canvas::Canvas) -> Result<isize, &'static str> {
    /*
    log!(
      "***tool_func start self.black_source={} self.black_token={} plines={}",
      self.black_source,
      self.black_token,
      self.plines.len()
    );
    */
    let margin = cv.met * 1.2 + cv.metr + cv.line_margin;

    match mt {
      // 1行進む
      FuncType::FdSlash => {
        let mut s: i32 = 0;

        for pl in &self.plines {
          match s {
            0 => {
              if pl.source == self.black_source {
                // 現在の行が見つかった。
                s = 1;
              }
            }
            1 => {
              // 次の行が見つかった。
              if self.black_source != -1 {
                self.black_source = pl.source;
                self.black_token = 0;
                s = 3;
                break;
              }
              s = 2;
            }
            _ => {
              // 次の行が見つかった。
              self.black_source = pl.source;
              self.black_token = 0;
              s = 3;
              break;
            }
          }
        }

        if s != 3 {
          if let Some(l) = self.plines.last() {
            self.black_source = l.source + 1;
            self.black_token = 0;
          }
        }

        let ai = self.area_index();

        if ai > -1 {
          let a = &self.areas[ai as usize];

          if self.is_vertical {
            if a.x1 < margin {
              self.pos += margin - a.x1;
            }
          } else {
            if a.y2 + margin > cv.height {
              self.pos += cv.height - margin - a.y2;
            }
          }
        }
      }

      // 1行戻る
      FuncType::BkSlash => {
        let mut last = false;
        if let Some(l) = self.plines.last() {
          if self.black_source > l.source {
            last = true;
            self.black_source = l.source;
            self.black_token = 0;
          }
        }
        if last == false {
          let mut s: i32 = 0;
          let mut j: usize = self.plines.len();

          loop {
            if j == 0 {
              break;
            }

            j -= 1;

            match s {
              0 => {
                if self.plines[j].source == self.black_source {
                  // 現在の行が見つかった。
                  s = 1;
                  if self.black_token != 0 {
                    self.black_token = 0;
                    break;
                  }
                }
              }
              _ => {
                // 前の行が見つかった。
                self.black_source = self.plines[j].source;
                self.black_token = 0;
                break;
              }
            }
          }
        }
      }

      // 1単語進む
      FuncType::FdOne => {
        let mut s: i32 = 0;

        for pl in &self.plines {
          if s == 0 && pl.source == self.black_source {
            // 現在の行が見つかった。
            s = 1;
          }

          if s > 0 {
            for vl in &pl.lines {
              for pt in &vl.ptokens {
                match s {
                  1 => {
                    if pt.seq == self.black_token {
                      // 現在のトークンが見つかった。
                      s = 2;

                      match pt.ty {
                        token::TokenType::Alpha
                        | token::TokenType::Hankigo
                        | token::TokenType::Kana
                        | token::TokenType::Yousoku
                        | token::TokenType::Zenkaku => {
                          // 現在の文字が見つかった。
                          s = 3;
                        }

                        _ => {}
                      }
                    }
                  }

                  2 => {
                    match pt.ty {
                      token::TokenType::Alpha
                      | token::TokenType::Hankigo
                      | token::TokenType::Kana
                      | token::TokenType::Yousoku
                      | token::TokenType::Zenkaku => {
                        // 現在の文字が見つかった。
                        s = 3;
                      }

                      _ => {}
                    }
                  }

                  3 => {
                    match pt.ty {
                      token::TokenType::Alpha
                      | token::TokenType::Hankigo
                      | token::TokenType::Kana
                      | token::TokenType::Yousoku
                      | token::TokenType::Zenkaku => {
                        // 次の文字が見つかった。
                        s = 4;
                        self.black_source = pl.source;
                        self.black_token = pt.seq;
                        let ai = self.area_index();

                        if ai > -1 {
                          let a = &self.areas[ai as usize];

                          if self.is_vertical {
                            if a.x1 < margin {
                              self.pos += margin - a.x1;
                            }
                          } else {
                            if a.y2 + margin > cv.height {
                              self.pos += cv.height - margin - a.y2;
                            }
                          }
                        }
                        break;
                      }

                      _ => {}
                    }
                  }

                  _ => {}
                }
              }

              if s == 4 {
                break;
              }
            }

            if s == 4 {
              break;
            }
          }
        }

        if s != 4 {
          if let Some(l) = self.plines.last() {
            self.black_source = l.source;
            self.black_token = l.ptokens.len() as isize;
          }

          let ai = self.area_index();

          if ai > -1 {
            let a = &self.areas[ai as usize];

            if self.is_vertical {
              if a.x1 < margin {
                self.pos += margin - a.x1;
              }
            } else {
              if a.y2 + margin > cv.height {
                self.pos += cv.height - margin - a.y2;
              }
            }
          }
        }
      }

      // 末尾に進む
      FuncType::FdBottom => {
        if let Some(l) = self.plines.last() {
          self.black_source = l.source + 1;
          self.black_token = 0;
        }

        let ai = self.area_index();

        if ai > -1 {
          let a = &self.areas[ai as usize];

          if self.is_vertical {
            if a.x1 < margin {
              self.pos += margin - a.x1;
            }
          } else {
            if a.y2 + margin > cv.height {
              self.pos += cv.height - margin - a.y2;
            }
          }
        }

        /*
        if self.is_vertical {
          self.pos = self.panel_width - (self.width * 0.6);

          if self.pos < 0.0 {
            self.pos = 0.0;
          }
        } else {
          self.pos = (self.height * 0.6) - self.panel_width;

          if self.pos > 0.0 {
            self.pos = 0.0;
          }
        }
        */
      }

      // 先頭に戻る
      FuncType::BkTop => {
        self.pos = 0.0;
        self.black_source = -1;
        self.black_token = 0;
      }

      _ => {}
    }

    Ok(0)
  }

  pub fn set_current(&mut self, cur: isize, cv: &canvas::Canvas) {
    self.current = cur;
    let mut count: usize = 0;
    for l in &self.plines {
      if l.source == self.current {
        break;
      }
      count = count + l.lines.len();
    }

    if self.is_vertical {
      self.pos = (cv.met * 1.2 + cv.metr + cv.line_margin) * count as f64;
    } else {
      self.pos = -(cv.met * 1.2 + cv.metr + cv.line_margin) * count as f64;
    }
  }

  fn area_index(&self) -> isize {
    let mut i: isize = 0;
    let mut j: isize = -1;

    for a in &self.areas {
      if a.source > self.black_source {
        break;
      }

      if a.source == self.black_source && a.token > self.black_token {
        break;
      }

      j = i;
      i += 1;
    }

    j
  }
}
