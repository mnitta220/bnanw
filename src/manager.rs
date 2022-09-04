use super::model::source;
use super::util;
use super::view;
use super::FuncType;
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
  pub is_black_text: bool,
  pub is_black_contents: bool,
  pub is_dark: bool,
  pub is_hide_text: bool,
  pub is_hide_contents: bool,
  pub is_hide_block: bool,
  pub section: isize,
  pub pcon: Option<view::panel_contents::PanelContents>,
  pub psec: Option<view::panel_section::PanelSection>,
  pub pbox: Option<view::panel_box::PanelBox>,
  pub pbd: Option<view::panel_board::PanelBoard>,
  pub tree: Option<super::model::contents::ContentTree>,
  pub canvas: Option<view::canvas::Canvas>,
  pub font_loaded: bool,
  pub width: i32,
  pub height: i32,
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
      is_black_text: false,
      is_black_contents: false,
      is_dark: false,
      is_hide_text: false,
      is_hide_contents: false,
      is_hide_block: false,
      section: 0,
      pcon: None,
      psec: None,
      pbox: None,
      pbd: None,
      tree: None,
      canvas: None,
      font_loaded: false,
      width: 0,
      height: 0,
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
    is_hide_block: bool,
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
    self.is_black_text = false;
    self.is_black_contents = false;
    self.is_hide_block = is_hide_block;
    self.font_size = font_size;
    self.section = current;
    self.sources.clear();
    self.contents.clear();
    self.canvas = None;
    self.pbox = None;
    self.pcon = Some(view::panel_contents::PanelContents::new());

    if let Some(pc) = &mut self.pcon {
      pc.black_source = current;
    }

    self.psec = Some(view::panel_section::PanelSection::new());
    self.pbd = Some(view::panel_board::PanelBoard::new());

    Ok(0)
  }

  /// 段落をセットする
  pub fn set_section(&mut self, current: isize) -> Result<isize, &'static str> {
    //log!("***Manager.set_section: current={}", current);
    self.tab = TabType::TabText;
    self.section = current;
    self.sources.clear();
    self.contents.clear();

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

  fn init_canvas(&mut self) -> Result<isize, &'static str> {
    let canvas: web_sys::HtmlCanvasElement;
    let context: web_sys::CanvasRenderingContext2d;

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

        canvas.set_width(self.width as u32);
        canvas.set_height(self.height as u32);
      }

      Err(_) => {
        return Err("ERR_GET_CANVAS");
      }
    }

    let cv = view::canvas::Canvas::new(
      canvas,
      context,
      self.width,
      self.height,
      self.is_vertical,
      self.font_size,
      self.font_loaded,
    );

    self.canvas = Some(cv);

    Ok(0)
  }

  /// 文書ツリーを生成する
  pub fn build_tree(&mut self) -> Result<isize, &'static str> {
    //log!("***Manager.build_tree");
    self.tree = Some(super::model::contents::ContentTree::build(&self));
    self.pbox = Some(view::panel_box::PanelBox::new(&self));
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
    self.width = width;
    self.height = height;
    self.is_dark = is_dark;

    if is_android {
      if self.font_loaded == false {
        return Err("ERR_GET_FONT");
      }
    }

    if let Err(e) = self.init_canvas() {
      return Err(e);
    }

    if let Some(pc) = &mut self.pcon {
      pc.black_source = self.section;
      pc.black_token = 0;
      pc.set_manager(
        self.is_vertical,
        self.font_size,
        &self.canvas,
        &self.contents,
        &self.sources,
      );
    }
    if let Some(ps) = &mut self.psec {
      ps.set_manager(self.is_vertical, self.font_size, &self.canvas);
    }
    if let Some(pb) = &mut self.pbd {
      pb.set_manager(&self.canvas);
    }

    if let Err(e) = self.change_section(self.section, true) {
      return Err(e);
    }

    if let Err(e) = self.draw() {
      return Err(e);
    }

    Ok(0)
  }

  /// キャンバスサイズ変更
  pub fn resize(&mut self, width: i32, height: i32, is_dark: bool) -> Result<isize, &'static str> {
    //log!("***Manager.resize");
    self.width = width;
    self.height = height;
    self.is_dark = is_dark;

    if let Err(e) = self.init_canvas() {
      return Err(e);
    }

    if let Some(pc) = &mut self.pcon {
      pc.black_source = self.section;
      pc.black_token = 0;
      pc.set_manager(
        self.is_vertical,
        self.font_size,
        &self.canvas,
        &self.contents,
        &self.sources,
      );
    }
    if let Some(ps) = &mut self.psec {
      ps.set_manager(self.is_vertical, self.font_size, &self.canvas);
    }
    if let Some(pb) = &mut self.pbd {
      pb.set_manager(&self.canvas);
    }

    if let Err(e) = self.change_section(self.section, false) {
      return Err(e);
    }

    if let Err(e) = self.draw() {
      return Err(e);
    }

    Ok(0)
  }

  /// タブを切り替える
  pub fn tab_change(
    &mut self,
    tab: TabType,
    width: i32,
    height: i32,
    is_dark: bool,
  ) -> Result<isize, &'static str> {
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

  /// 現在のセクションを返す
  pub fn get_section(&mut self) -> isize {
    //log!("***Manager.get_section");
    self.section
  }

  /// 文書を表示する。
  fn draw(&mut self) -> Result<isize, &'static str> {
    //log!("***Manager.draw");
    if self.canvas.is_none() {
      if let Err(e) = self.init_canvas() {
        return Err(e);
      }
    }

    match self.tab {
      TabType::TabContents => {
        if let Some(pc) = &mut self.pcon {
          if let Some(cv) = &self.canvas {
            let mut areas: Vec<view::area::Area> = Vec::new();

            if let Err(e) = pc.draw(
              &cv,
              &mut areas,
              self.is_black_contents,
              self.is_dark,
              self.is_hide_contents,
              self.is_hide_block,
            ) {
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

            if let Err(e) = ps.draw(
              &cv,
              &mut areas,
              self.is_black_text,
              self.is_dark,
              self.is_hide_text,
              self.is_hide_block,
            ) {
              return Err(e);
            }

            ps.areas.clear();
            ps.areas = areas;
          } else {
            return Err("ERR_GET_CANVAS");
          }
        }
      }
      TabType::TabBox => {
        if let Some(bx) = &mut self.pbox {
          if let Some(tr) = &mut self.tree {
            if let Some(cv) = &self.canvas {
              let mut areas: Vec<view::area::Area> = Vec::new();

              if let Err(e) = bx.draw(tr, &cv, &mut areas, self.is_black_text, self.is_dark, 32) {
                return Err(e);
              }

              bx.areas.clear();
              bx.areas = areas;
            } else {
              return Err("ERR_GET_CANVAS");
            }
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
          if let Err(e) = ps.touch_start(x, y) {
            return Err(e);
          }
        }
      }
      TabType::TabBox => {
        if let Some(bx) = &mut self.pbox {
          if let Err(e) = bx.touch_start(x, y) {
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
      TabType::TabBox => {
        if let Some(bx) = &mut self.pbox {
          match bx.touch_move(x, y) {
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
            Ok(r) => {
              if r > -2 {
                ret = r;
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
          if let Err(e) = ps.touch_end() {
            return Err(e);
          }
        }
      }

      TabType::TabBox => {
        if let Some(bx) = &mut self.pbox {
          match bx.touch_end() {
            Ok(r) => {
              if self.is_black_text && r == -2 {
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

  /// シングルクリック
  pub fn single_click(&mut self, x: i32, y: i32) -> Result<isize, &'static str> {
    let mut ret: isize = -3;

    match self.tab {
      TabType::TabContents => {
        if let Some(pc) = &mut self.pcon {
          match pc.single_click(x, y) {
            Ok(r) => {
              ret = r;
            }

            Err(e) => {
              return Err(e);
            }
          }
        }
      }
      TabType::TabText => {
        if let Some(ps) = &mut self.psec {
          match ps.single_click(x, y) {
            Ok(r) => {
              ret = r;
            }

            Err(e) => {
              return Err(e);
            }
          }
        }
      }
      _ => {}
    }

    Ok(ret)
  }

  /// ダブルクリック
  pub fn double_click(&mut self, x: i32, y: i32) -> Result<isize, &'static str> {
    let mut ret: isize = -3;

    match self.tab {
      TabType::TabContents => {
        if let Some(pc) = &mut self.pcon {
          match pc.double_click(x, y) {
            Ok(r) => {
              ret = r;

              if self.is_black_contents {
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
          match ps.double_click(x, y) {
            Ok(r) => {
              ret = r;

              if self.is_black_text {
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
      _ => {}
    }

    Ok(ret)
  }

  /// 黒塗りモードを変更する
  pub fn mode_change(&mut self, black: bool) -> Result<isize, &'static str> {
    //log!("***mode_change black={}", black);
    match self.tab {
      TabType::TabText => {
        self.is_black_text = black;
        self.draw()
      }
      TabType::TabContents => {
        self.is_black_contents = black;
        self.draw()
      }
      _ => Ok(0),
    }
  }

  fn change_section(&mut self, section: isize, black_init: bool) -> Result<isize, &'static str> {
    //log!("***Manager.change_section: section={}", section);
    if self.canvas.is_none() {
      if let Err(e) = self.init_canvas() {
        return Err(e);
      }
    }

    if let Some(pc) = &mut self.pcon {
      if let Some(cv) = &self.canvas {
        pc.set_current(section, &cv);
      }
    }

    if let Some(ps) = &mut self.psec {
      ps.plines.clear();

      if black_init {
        ps.black_source = -1;
        ps.black_token = 0;
      }

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

            //log!("***change_section PanelLine");
            let pl = view::panel_line::PanelLine::new(self.is_vertical, &s, &cv);
            ps.plines.push(pl);
          }
        }

        self.section = section;
        let panel_width = (cv.met * 1.2 + cv.metr + cv.line_margin) * ps.count_lines() as f64;
        ps.set_panel_width(panel_width);
      } else {
        return Err("ERR_GET_CANVAS");
      }

      if black_init {
        ps.pos = 0.0;
      }
    }

    Ok(0)
  }

  /// ツールボタンの操作
  pub fn tool_func(&mut self, mt: FuncType) -> Result<isize, &'static str> {
    /* log!("***manager::tool_func mt={}", mt); */
    if self.canvas.is_none() {
      if let Err(e) = self.init_canvas() {
        return Err(e);
      }
    }

    match self.tab {
      TabType::TabText => {
        if let Some(ps) = &mut self.psec {
          if self.is_black_text {
            if let Some(cv) = &self.canvas {
              if let Err(e) = ps.tool_func(mt, &cv) {
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
              // 1ページ進む
              FuncType::FdSlash => {
                if let Some(cv) = &self.canvas {
                  let l = ps.count_lines();
                  //let w = cv.met * 1.2 + cv.metr + cv.line_margin;
                  if self.is_vertical {
                    /*
                    log!(
                      "***pos={} width={} panel_width={}",
                      ps.pos,
                      ps.width,
                      ps.panel_width
                    );
                    */
                    if cv.line_width * l as f64 > (ps.pos + ps.width) {
                      let mut cur_line = (ps.pos / cv.line_width) as i32;
                      cur_line += cv.page_lines;
                      ps.pos = cv.line_width * cur_line as f64;

                      //ps.pos += ps.width;
                      //let c = (ps.pos / cv.line_width) as i32;
                      //ps.pos = cv.line_width * (c as f64);
                    }

                    //if ps.pos < 0.0 {
                    //  ps.pos = 0.0;
                    //}
                  } else {
                    //log!("***pos={} height={} w={} l={}", ps.pos, ps.height, w, l);
                    if cv.line_width * l as f64 > (-ps.pos + ps.height) {
                      ps.pos = ps.pos - ps.height;
                      let c = (ps.pos / cv.line_width) as i32;
                      ps.pos = cv.line_width * (c as f64);
                    }
                    //log!("***pos={}", ps.pos);
                    /*
                    ps.pos = ps.pos + ps.height;

                    if ps.pos > 0.0 {
                      ps.pos = 0.0;
                    }
                    */
                  }

                  if let Err(e) = self.draw() {
                    return Err(e);
                  }
                } else {
                  return Err("ERR_GET_CANVAS");
                }
              }

              // 1ページ戻る
              FuncType::BkSlash => {
                if let Some(cv) = &self.canvas {
                  //let w = cv.met * 1.2 + cv.metr + cv.line_margin;
                  if self.is_vertical {
                    let mut cur_line = (ps.pos / cv.line_width) as i32;
                    if cur_line > cv.page_lines {
                      cur_line -= cv.page_lines;
                    } else {
                      cur_line = 0;
                    }
                    ps.pos = cv.line_width * cur_line as f64;
                    /*
                    //let page_lines =
                    ps.pos -= ps.width;
                    let c = (ps.pos / cv.line_width) as i32;
                    ps.pos = cv.line_width * (c as f64);

                    if ps.pos < 0.0 {
                      ps.pos = 0.0;
                    }
                    */
                  } else {
                    ps.pos += ps.height;
                    let c = (ps.pos / cv.line_width) as i32;
                    ps.pos = cv.line_width * (c as f64);

                    if ps.pos > 0.0 {
                      ps.pos = 0.0;
                    }
                  }

                  if let Err(e) = self.draw() {
                    return Err(e);
                  }
                } else {
                  return Err("ERR_GET_CANVAS");
                }
              }

              // 末尾に進む
              FuncType::FdBottom => {
                if let Some(cv) = &self.canvas {
                  let l = ps.count_lines();
                  if self.is_vertical {
                    //if cv.line_width * l as f64 > (ps.pos + ps.width) {
                    let mut cur_line = (ps.pos / cv.line_width) as usize;
                    loop {
                      if cur_line + cv.page_lines as usize >= l {
                        break;
                      }
                      cur_line += cv.page_lines as usize;
                    }
                    ps.pos = cv.line_width * cur_line as f64;
                    //}
                  } else {
                    //log!("***pos={} height={} w={} l={}", ps.pos, ps.height, w, l);
                  }

                  if let Err(e) = self.draw() {
                    return Err(e);
                  }
                } else {
                  return Err("ERR_GET_CANVAS");
                }
                /*
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
                */
              }

              // 先頭に戻る
              FuncType::BkTop => {
                ps.pos = 0.0;

                if let Err(e) = self.draw() {
                  return Err(e);
                }
              }

              // 原稿用紙非表示
              FuncType::HideBlock => {
                self.is_hide_block = true;

                if let Err(e) = self.draw() {
                  return Err(e);
                }
              }

              // 原稿用紙表示
              FuncType::ShowBlock => {
                self.is_hide_block = false;

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
          if let Some(cv) = &self.canvas {
            if let Err(e) = pc.tool_func(mt, &cv) {
              return Err(e);
            }

            if let Err(e) = self.draw() {
              return Err(e);
            }
          } else {
            return Err("ERR_GET_CANVAS");
          }
        }
      }

      TabType::TabBox => {
        if let Some(bx) = &mut self.pbox {
          if let Err(e) = bx.tool_func(mt) {
            return Err(e);
          }

          if let Err(e) = self.draw() {
            return Err(e);
          }
        }
      }
      TabType::TabBoard => {}
    }

    Ok(0)
  }

  /// 非表示
  pub fn hide(&mut self, is_hide: bool) -> Result<isize, &'static str> {
    //log!("***manager::hide is_hide={}", is_hide);
    if self.canvas.is_none() {
      if let Err(e) = self.init_canvas() {
        return Err(e);
      }
    }

    match self.tab {
      TabType::TabText => {
        self.is_hide_text = is_hide;
        if let Err(e) = self.draw() {
          return Err(e);
        }
      }

      TabType::TabContents => {
        self.is_hide_contents = is_hide;
        if let Err(e) = self.draw() {
          return Err(e);
        }
      }
      _ => {}
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
