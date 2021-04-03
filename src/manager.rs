use super::model::source;
use super::util;
use super::view;
use super::MoveType;
use super::TabType;
use crate::view::panel::Panel;

pub const DOC_TOP: isize = -1;

pub struct Manager {
  pub id: isize,
  pub title: String,
  pub is_vertical: bool,
  pub font_size: isize,
  pub sources: Vec<source::Source>,
  pub contents: Vec<usize>,
  pub tab: TabType,
  pub is_black: bool,
  pub is_dark: bool,
  pub section: isize,
  pub pcon: Option<view::panel_contents::PanelContents>,
  pub psec: Option<view::panel_section::PanelSection>,
  pub pbd: Option<view::panel_board::PanelBoard>,
  pub canvas: Option<view::canvas::Canvas>,
  pub click_handle: Vec<super::click::ClickHandle>,
  pub font_loaded: bool,
}

impl Manager {
  pub fn new() -> Self {
    Manager {
      id: 0,
      title: String::new(),
      is_vertical: false,
      font_size: 0,
      sources: Vec::new(),
      contents: Vec::new(),
      tab: TabType::TabText,
      is_black: false,
      is_dark: false,
      section: 0,
      pcon: None,
      psec: None,
      pbd: None,
      canvas: None,
      click_handle: Vec::new(),
      font_loaded: false,
    }
  }

  /// 疎通確認
  pub fn ping(&self, input: isize) -> isize {
    input + 1
  }

  /// 文書をセットする
  pub fn set_doc(
    &mut self,
    id: isize,
    title: &str,
    vertical: isize,
    font_size: isize,
    current: isize,
  ) -> Result<isize, &'static str> {
    //log!("***Manager.set_doc: id={} current={}", id, current);
    self.id = id;
    self.title = String::from(title);
    self.tab = TabType::TabText;
    let mut is_vertical = false;

    if vertical == 2 {
      is_vertical = true;
    }

    self.is_vertical = is_vertical;
    self.is_black = false;
    self.font_size = font_size;
    self.section = current;
    self.sources.clear();
    self.contents.clear();
    self.canvas = None;
    self.pcon = None;
    self.psec = None;
    self.pbd = None;

    Ok(0)
  }

  /// 文書の行をセットする
  pub fn set_source(&mut self, seq: isize, text: &str) -> Result<isize, &'static str> {
    //log!("***Manager.set_source: seq={}, text={}", seq, text);
    let source = source::Source::new(seq, text);

    if source.ty != 0 {
      self.contents.push(self.sources.len());
    }

    self.sources.push(source);

    Ok(0)
  }

  /// 文書を表示する
  pub fn draw_doc(
    &mut self,
    width: i32,
    height: i32,
    is_dark: bool,
    is_android: bool,
  ) -> Result<isize, &'static str> {
    //log!("***Manager.draw_doc");
    if is_android {
      if self.font_loaded == false {
        return Err("ERR_GET_FONT");
      }
    }

    let canvas: web_sys::HtmlCanvasElement;
    let context: web_sys::CanvasRenderingContext2d;
    self.is_dark = is_dark;

    match util::get_canvas("ca1") {
      Ok(c) => {
        match util::get_context(&c) {
          Ok(cn) => {
            context = cn;
          }

          Err(_) => {
            return Err("ERR_GET_CONTEXT");
          }
        }

        canvas = c;

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
      }

      Err(_) => {
        return Err("ERR_GET_CANVAS");
      }
    }

    let cv = view::canvas::Canvas::new(
      canvas,
      context,
      width,
      height,
      self.is_vertical,
      self.font_size,
      self.font_loaded,
    );

    self.canvas = Some(cv);
    self.pcon = Some(view::panel_contents::PanelContents::new(&self));
    self.psec = Some(view::panel_section::PanelSection::new(&self));
    self.pbd = Some(view::panel_board::PanelBoard::new(&self));

    if let Err(e) = self.change_section(self.section) {
      return Err(e);
    }

    self.click_handle.clear();

    if let Err(e) = self.draw() {
      return Err(e);
    }

    Ok(0)
  }

  /// キャンバスサイズ変更
  pub fn resize(&mut self, width: i32, height: i32, is_dark: bool) -> Result<isize, &'static str> {
    //log!("***Manager.resize");
    let canvas: web_sys::HtmlCanvasElement;
    let context: web_sys::CanvasRenderingContext2d;
    self.is_dark = is_dark;

    match util::get_canvas("ca1") {
      Ok(c) => {
        match util::get_context(&c) {
          Ok(cn) => {
            context = cn;
          }

          Err(e) => {
            return Err(e);
          }
        }

        canvas = c;

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
      }

      Err(e) => {
        return Err(e);
      }
    }

    let cv = view::canvas::Canvas::new(
      canvas,
      context,
      width,
      height,
      self.is_vertical,
      self.font_size,
      self.font_loaded,
    );

    self.canvas = Some(cv);
    let mut pos: f64 = 0.0;

    if let Some(pc) = &self.pcon {
      pos = pc.pos;
    }

    let mut pc = view::panel_contents::PanelContents::new(&self);
    pc.pos = pos;
    self.pcon = Some(pc);

    let mut pos: f64 = 0.0;
    let mut black_source: isize = 0;
    let mut black_token: isize = 0;

    if let Some(ps) = &self.psec {
      pos = ps.pos;
      black_source = ps.black_source;
      black_token = ps.black_token;
    }

    let ps = view::panel_section::PanelSection::new(&self);
    self.psec = Some(ps);

    if let Err(e) = self.change_section(self.section) {
      return Err(e);
    }

    if let Some(ps) = &mut self.psec {
      ps.pos = pos;
      ps.black_source = black_source;
      ps.black_token = black_token;
    }

    self.click_handle.clear();

    if let Err(e) = self.draw() {
      return Err(e);
    }

    Ok(0)
  }

  /// タブを切り替える
  pub fn tab_change(&mut self, tab: TabType, width: i32, height: i32, is_dark: bool) -> Result<isize, &'static str> {
    //log!("***Manager.tab_change tab={}", tab);
    self.tab = tab;
    let canvas: web_sys::HtmlCanvasElement;
    let context: web_sys::CanvasRenderingContext2d;
    self.is_dark = is_dark;

    match util::get_canvas("ca1") {
      Ok(c) => {
        match util::get_context(&c) {
          Ok(cn) => {
            context = cn;
          }

          Err(e) => {
            return Err(e);
          }
        }

        canvas = c;

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
      }

      Err(e) => {
        return Err(e);
      }
    }

    let cv = view::canvas::Canvas::new(
      canvas,
      context,
      width,
      height,
      self.is_vertical,
      self.font_size,
      self.font_loaded,
    );

    self.canvas = Some(cv);

    if let Some(c) = &self.canvas {
      c.clear(self.is_dark);
    }

    if let Err(e) = self.draw() {
      return Err(e);
    }

    Ok(0)
  }

  /// 文書を表示する。
  fn draw(&mut self) -> Result<isize, &'static str> {
    //log!("***Manager.draw");
    match self.tab {
      TabType::TabContents => {
        if let Some(pc) = &mut self.pcon {
          if let Some(cv) = &self.canvas {
            let mut areas: Vec<view::area::Area> = Vec::new();

            if let Err(e) = pc.draw(&cv, &mut areas, self.is_black, self.is_dark) {
              return Err(e);
            }

            pc.areas.clear();
            pc.areas = areas;
          } else {
            return Err("ERR_GET_CANVAS");
          }
        }
      }
      TabType::TabText => {
        if let Some(ps) = &mut self.psec {
          if let Some(cv) = &self.canvas {
            let mut areas: Vec<view::area::Area> = Vec::new();

            if let Err(e) = ps.draw(&cv, &mut areas, self.is_black, self.is_dark) {
              return Err(e);
            }

            ps.areas.clear();
            ps.areas = areas;
          } else {
            return Err("ERR_GET_CANVAS");
          }
        }
      }
      TabType::TabBoard => {
        if let Some(bd) = &mut self.pbd {
          if let Some(cv) = &self.canvas {
            if let Err(e) = bd.draw(&cv, self.is_dark) {
              return Err(e);
            }
          } else {
            return Err("ERR_GET_CANVAS");
          }
        }
      }
    }

    Ok(0)
  }

  /// タッチ開始
  pub fn touch_start(&mut self, x: i32, y: i32) -> Result<isize, &'static str> {
    match self.tab {
      TabType::TabContents => {
        if let Some(pc) = &mut self.pcon {
          if let Err(e) = pc.touch_start(x, y) {
            return Err(e);
          }
        }
      }
      TabType::TabText => {
        if let Some(ps) = &mut self.psec {
          let handle = super::click_handle();
          self.click_handle.push(handle);

          if let Err(e) = ps.touch_start(x, y) {
            return Err(e);
          }
        }
      }
      TabType::TabBoard => {
        if let Some(bd) = &mut self.pbd {
          if let Err(e) = bd.touch_start(x, y) {
            return Err(e);
          }
        }
      }
    }

    Ok(0)
  }

  /// タッチを移動する
  pub fn touch_move(&mut self, x: i32, y: i32) -> Result<isize, &'static str> {
    match self.tab {
      TabType::TabContents => {
        if let Some(pc) = &mut self.pcon {
          match pc.touch_move(x, y) {
            Ok(r) => {
              if r == 0 {
                if let Err(e) = self.draw() {
                  return Err(e);
                }
              }
            }

            Err(e) => {
              return Err(e);
            }
          }
        }
      }
      TabType::TabText => {
        if let Some(ps) = &mut self.psec {
          match ps.touch_move(x, y) {
            Ok(r) => {
              if r == 0 {
                if let Err(e) = self.draw() {
                  return Err(e);
                }
              }
            }

            Err(e) => {
              return Err(e);
            }
          }
        }
      }
      TabType::TabBoard => {
        if let Some(bd) = &mut self.pbd {
          if let Err(e) = bd.touch_move(x, y) {
            return Err(e);
          }
          if let Err(e) = self.draw() {
            return Err(e);
          }
        }
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
    //log!("***Manager.touch_end");
    let mut ret: isize = -3;

    match self.tab {
      TabType::TabContents => {
        if let Some(pc) = &mut self.pcon {
          match pc.touch_end() {
            Ok(i) => {
              if i > -2 {
                // 目次選択
                if let Err(e) = self.change_section(i) {
                  return Err(e);
                }

                ret = i;
              }
            }

            Err(e) => {
              return Err(e);
            }
          }
        }
      }
      TabType::TabText => {
        if let Some(ps) = &mut self.psec {
          match ps.touch_end() {
            Ok(r) => {
              if self.is_black && r == -2 {
                if let Err(e) = self.draw() {
                  return Err(e);
                }
              }
            }

            Err(e) => return Err(e),
          }
        }
      }
      TabType::TabBoard => {
        if let Some(bd) = &mut self.pbd {
          if let Err(e) = bd.touch_end() {
            return Err(e);
          }
        }
      }
    }

    Ok(ret)
  }

  /// クリック
  pub fn click(&mut self) -> Result<isize, &'static str> {
    //log!("***clicked!");

    match self.tab {
      TabType::TabContents => {
        if let Some(pc) = &mut self.pcon {
          if let Err(e) = pc.click() {
            return Err(e);
          }
        }
      }
      TabType::TabText => {
        if let Some(ps) = &mut self.psec {
          match ps.click() {
            Ok(r) => {
              if r == 1 {
                if let Err(e) = self.black_step(MoveType::FdSlash) {
                  return Err(e);
                }
              }
            }

            Err(e) => {
              return Err(e);
            }
          }
        }
      }
      TabType::TabBoard => {}
    }

    Ok(0)
  }

  /// 黒塗りモードを変更する
  pub fn mode_change(&mut self, black: bool) -> Result<isize, &'static str> {
    //log!("***mode_change black={}", black);
    match self.tab {
      TabType::TabText => {
        self.is_black = black;
        self.draw()
      }
      _ => Ok(0),
    }
  }

  fn change_section(&mut self, section: isize) -> Result<isize, &'static str> {
    //log!("***Manager.change_section: section={}", section);

    if let Some(pc) = &mut self.pcon {
      pc.current = section;
    }

    if let Some(ps) = &mut self.psec {
      ps.plines.clear();

      ps.black_source = -1;
      ps.black_token = 0;

      if let Some(cv) = &self.canvas {
        let mut ty: isize = 0;
        let mut is_text: bool = false;
        let mut found = false;
        let mut sec = section;

        if sec == DOC_TOP {
          sec = 0;
        }

        for s in &self.sources {
          if s.seq == sec {
            found = true;
          }

          if found {
            if s.ty == 0 {
              is_text = true;

              if ps.black_source == -1 {
                ps.black_source = s.seq;
              }
            } else {
              if is_text {
                break;
              }

              if s.ty <= ty {
                break;
              }

              ty = s.ty;
            }

            let pl = view::panel_line::PanelLine::new(self.is_vertical, &s, &cv);
            ps.plines.push(pl);
          }
        }

        self.section = section;
        let panel_width = (cv.met + cv.ruby_w + cv.line_margin) * ps.count_lines() as f64;
        ps.set_panel_width(panel_width);
      } else {
        return Err("ERR_GET_CANVAS");
      }

      ps.pos = 0.0;
    }

    Ok(0)
  }

  /// 黒塗りを移動する
  pub fn black_step(&mut self, mt: MoveType) -> Result<isize, &'static str> {
    //log!("***black_step {}", step);
    match self.tab {
      TabType::TabText => {
        if let Some(ps) = &mut self.psec {
          if self.is_black {
            if let Some(cv) = &self.canvas {
              if let Err(e) = ps.black_step(mt, &cv) {
                return Err(e);
              }

              if let Err(e) = self.draw() {
                return Err(e);
              }
            } else {
              return Err("ERR_GET_CANVAS");
            }
          } else {
            match mt {
              // 末尾に進む
              MoveType::FdBottom => {
                if self.is_vertical {
                  ps.pos = ps.panel_width - (ps.width * 0.6);

                  if ps.pos < 0.0 {
                    ps.pos = 0.0;
                  }
                } else {
                  ps.pos = (ps.height * 0.6) - ps.panel_width;

                  if ps.pos > 0.0 {
                    ps.pos = 0.0;
                  }
                }

                if let Err(e) = self.draw() {
                  return Err(e);
                }
              }

              // 先頭に戻る
              MoveType::BkTop => {
                ps.pos = 0.0;

                if let Err(e) = self.draw() {
                  return Err(e);
                }
              }

              _ => {}
            }
          }
        }
      }
      TabType::TabContents => {
        if let Some(pc) = &mut self.pcon {
          match mt {
            // 末尾に進む
            MoveType::FdBottom => {
              if self.is_vertical {
                pc.pos = pc.panel_width - (pc.width * 0.6);

                if pc.pos < 0.0 {
                  pc.pos = 0.0;
                }
              } else {
                pc.pos = (pc.height * 0.6) - pc.panel_width;

                if pc.pos > 0.0 {
                  pc.pos = 0.0;
                }
              }

              if let Err(e) = self.draw() {
                return Err(e);
              }
            }

            // 先頭に戻る
            MoveType::BkTop => {
              pc.pos = 0.0;

              if let Err(e) = self.draw() {
                return Err(e);
              }
            }

            _ => {}
          }
        }
      }
      TabType::TabBoard => {}
    }

    Ok(0)
  }

  /// 白板・戻る
  pub fn stroke_back(&mut self) -> Result<isize, &'static str> {
    //log!("***stroke_back");
    match self.tab {
      TabType::TabBoard => {
        if let Some(bd) = &mut self.pbd {
          if let Err(e) = bd.stroke_back() {
            return Err(e);
          }
          if let Err(e) = self.draw() {
            return Err(e);
          }
        }
      }
      _ => {}
    }

    Ok(0)
  }

  /// 白板・消去
  pub fn stroke_clear(&mut self) -> Result<isize, &'static str> {
    //log!("***stroke_clear");
    match self.tab {
      TabType::TabBoard => {
        if let Some(bd) = &mut self.pbd {
          if let Err(e) = bd.stroke_clear() {
            return Err(e);
          }
          if let Err(e) = self.draw() {
            return Err(e);
          }
        }
      }
      _ => {}
    }

    Ok(0)
  }
}
