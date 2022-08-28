#[macro_use]
mod util;

mod manager;
mod model;
mod view;

use std::cell::RefCell;
use strum_macros::Display;
use wasm_bindgen::prelude::*;
use web_sys::FontFace;

// ツールボタン操作タイプ
#[derive(Display, Debug)]
pub enum FuncType {
  // 1区切り進む
  FdSlash,
  // 1区切り戻る
  BkSlash,
  // 1単語進む
  FdOne,
  // 末尾に進む
  FdBottom,
  // 先頭に戻る
  BkTop,
  // 次の段・節に進む
  FdSec,
  // 前の段・節に戻る
  BkSec,
  // 原稿用紙非表示
  HideBlock,
  // 原稿用紙表示
  ShowBlock,
}

#[derive(Display, Debug)]
pub enum TabType {
  // 本文
  TabText,
  // 目次
  TabContents,
  // 白板
  TabBoard,
  // 9Box
  TabBox,
}

// 画面の情報をスレッドローカルのスタティックに保持する
thread_local!(
  pub static MANAGER: RefCell<manager::Manager> = RefCell::new(manager::Manager::new())
);

// Androidには明朝体フォントが入っていないため、Googleフォントの Noto Serif JP を使用する。
// https://fonts.google.com/specimen/Noto+Serif+JP
thread_local!(
  pub static GOOGLE_FONT: RefCell<FontFace> =
  RefCell::new(FontFace::new_with_str("googleFont",
   "url(/assets/font/NotoSerifJP-Regular.otf)").unwrap())
);

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
  fn setTimeout(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
}

/// 起動時処理
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  Ok(())
}

/// 疎通確認
///
/// # 引数
/// input
///
/// # 戻り値
/// - input + 1
///
#[wasm_bindgen]
pub fn ping(input: isize) -> Result<isize, JsValue> {
  // log!("***ping: input={}", input);
  Ok(MANAGER.with(|mg| mg.borrow().ping(input)))
}

/// Googleフォントロード処理
///
/// # 引数
/// なし
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn load_font() -> Result<(), JsValue> {
  if let Err(e) = load_font_sub() {
    return Err(JsValue::from_str(&format!("load_font failed!: {}", e)));
  }

  Ok(())
}

fn load_font_sub() -> Result<(), &'static str> {
  GOOGLE_FONT.with(|gf| {
    let f = gf.borrow_mut();

    match f.load() {
      Ok(p) => {
        wasm_bindgen_futures::spawn_local(async {
          match wasm_bindgen_futures::JsFuture::from(p).await {
            Ok(_) => {
              MANAGER.with(|mg| match mg.try_borrow_mut() {
                Ok(mut m) => {
                  m.font_loaded = true;
                  m.canvas = None;
                }

                Err(_) => {}
              });
            }

            Err(_) => {}
          }
        });
      }

      Err(e) => return e,
    }

    match util::get_document() {
      Ok(d) => {
        match d.fonts().add(&f) {
          Ok(_) => {}

          Err(_) => {}
        }

        wasm_bindgen::JsValue::TRUE
      }

      Err(_) => wasm_bindgen::JsValue::FALSE,
    }
  });

  Ok(())
}

/// 文書をセットする
///
/// # 引数
/// ## id
/// ## title
/// ## vertical
/// - 1 : 横書き
/// - 2 : 縦書き
/// ## font_size
/// ## current
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn set_doc(
  id: isize,
  title: &str,
  vertical: isize,
  font_size: isize,
  current: isize,
  is_hide_block: bool,
) -> Result<(), JsValue> {
  /*
  log!(
    "***set_doc: id={}, title={}, vertical={} font_size={}, current={}",
    id,
    title,
    vertical,
    font_size,
    current
  );
  */
  if let Err(e1) = MANAGER.with(|mg| {
    match mg
      .borrow_mut()
      .set_doc(id, title, vertical, font_size, current, is_hide_block)
    {
      Err(e) => {
        return Err(JsValue::from_str(&format!("set_doc failed!: {}", e)));
      }

      _ => Ok(()),
    }
  }) {
    return Err(JsValue::from_str(&format!("set_doc failed!: {:?}", e1)));
  }

  Ok(())
}

/// 段落をセットする
///
/// # 引数
/// ## current
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn set_section(current: isize) -> Result<(), JsValue> {
  //log!("***set_section: current={}", current);
  if let Err(e1) = MANAGER.with(|mg| match mg.borrow_mut().set_section(current) {
    Err(e) => {
      return Err(JsValue::from_str(&format!("set_section failed!: {}", e)));
    }

    _ => Ok(()),
  }) {
    return Err(JsValue::from_str(&format!("set_section failed!: {:?}", e1)));
  }

  Ok(())
}

/// 文書の行をセットする
///
/// # 引数
/// ## seq
/// ## text
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn set_source(seq: isize, text: &str) -> Result<(), JsValue> {
  //log!("***set_source: seq={}, text={}", seq, text);

  if let Err(e1) = MANAGER.with(|mg| match mg.borrow_mut().set_source(seq, text) {
    Err(e) => {
      return Err(JsValue::from_str(&format!("set_source failed!: {}", e)));
    }

    _ => Ok(()),
  }) {
    return Err(JsValue::from_str(&format!("set_source failed!: {:?}", e1)));
  }

  Ok(())
}

/// 文書ツリーを生成する
///
/// # 引数
/// なし
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn build_tree() -> Result<(), JsValue> {
  //log!("***build_tree");
  if let Err(e1) = MANAGER.with(|mg| match mg.borrow_mut().build_tree() {
    Err(e) => {
      return Err(JsValue::from_str(&format!("build_tree failed!: {}", e)));
    }

    _ => Ok(()),
  }) {
    return Err(JsValue::from_str(&format!("build_tree failed!: {:?}", e1)));
  }

  Ok(())
}

/// 文書を表示する
///
/// # 引数
/// ## width
/// - キャンバスの幅
/// ## height
/// - キャンバスの高さ
/// ## is_dark
/// - true: ダークモード
/// ## is_android
/// - true: Android
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn draw_doc(width: i32, height: i32, is_dark: bool, is_android: bool) -> Result<(), JsValue> {
  //log!("***draw_doc: width={} height={}", width, height);

  if let Err(e1) =
    MANAGER.with(
      |mg| match mg.borrow_mut().draw_doc(width, height, is_dark, is_android) {
        Err(e) => {
          if e == "FONT_NOT_LOAD_YET" {
            return Err(JsValue::from_str(e));
          }

          return Err(JsValue::from_str(&format!("draw_doc failed!: {}", e)));
        }

        _ => Ok(()),
      },
    )
  {
    return Err(JsValue::from_str(&format!("draw_doc failed!: {:?}", e1)));
  }

  Ok(())
}

/// キャンバスサイズ変更
///
/// # 引数
/// ## width
/// - キャンバスの幅
/// ## height
/// - キャンバスの高さ
/// ## is_dark
/// - true: ダークモード
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn resize(width: i32, height: i32, is_dark: bool) -> Result<(), JsValue> {
  //log!("***resize: width={} height={}", width, height);

  if let Err(e1) = MANAGER.with(|mg| match mg.borrow_mut().resize(width, height, is_dark) {
    Err(e) => {
      return Err(JsValue::from_str(&format!("resize failed!: {}", e)));
    }

    _ => Ok(()),
  }) {
    return Err(JsValue::from_str(&format!("resize failed!: {:?}", e1)));
  }

  Ok(())
}

/// タブを切り替える
///
/// # 引数
/// ## tab
/// - 0: 本文
/// - 1: 目次
/// - 2: Box
/// - 3: 白板
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn tab_change(tab: i32, width: i32, height: i32, is_dark: bool) -> Result<isize, JsValue> {
  //log!("***tab_change tab={}", tab);
  let mut ret: isize = -1;
  let t: TabType;
  match tab {
    1 => t = TabType::TabContents,
    2 => t = TabType::TabBox,
    3 => t = TabType::TabBoard,
    _ => t = TabType::TabText,
  }

  if let Err(e1) = MANAGER.with(
    |mg| match mg.borrow_mut().tab_change(t, width, height, is_dark) {
      Ok(r) => {
        ret = r;

        Ok(ret)
      }

      Err(e) => {
        return Err(JsValue::from_str(&format!("tab_change failed!: {}", e)));
      }
    },
  ) {
    return Err(JsValue::from_str(&format!("tab_change failed!: {:?}", e1)));
  }

  Ok(ret)
}

/// 現在のセクションを返す
///
/// # 引数
/// なし
///
/// # 戻り値
/// セクション
///
#[wasm_bindgen]
pub fn get_section() -> isize {
  //log!("***get_section");
  MANAGER.with(|mg| mg.borrow_mut().get_section())
}

/// タッチ開始
///
/// # 引数
/// ## x
/// ## y
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn touch_start(x: i32, y: i32) -> Result<(), JsValue> {
  //log!("***touch_start: x={} y={}", x, y);
  //let ret: isize = 0;

  if let Err(e1) = MANAGER.with(|mg| match mg.try_borrow_mut() {
    Ok(mut m) => match m.touch_start(x, y) {
      Err(e) => {
        return Err(JsValue::from_str(&format!("touch_start failed!: {}", e)));
      }

      _ => Ok(()),
    },

    Err(e) => {
      return Err(JsValue::from_str(&format!("touch_start failed!: {}", e)));
    }
  }) {
    return Err(JsValue::from_str(&format!("touch_start failed!: {:?}", e1)));
  }

  Ok(())
}

/// タッチを移動する
///
/// # 引数
/// ## x
/// ## y
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn touch_move(x: i32, y: i32) -> Result<(), JsValue> {
  //log!("***touch_move: x={} y={}", x, y);

  MANAGER.with(|mg| match mg.try_borrow_mut() {
    Ok(mut m) => match m.touch_move(x, y) {
      Err(_) => {}

      _ => {}
    },

    Err(_) => {}
  });

  Ok(())
}

/// タッチ終了
///
/// # 引数
/// なし
///
/// # 戻り値
/// - -2 : 正常終了
/// - -1 : Top選択
/// - 0以上 : セクション選択
/// - それ以外 : 異常終了
///
#[wasm_bindgen]
pub fn touch_end() -> Result<isize, JsValue> {
  //log!("***lib.touch_end");
  let mut ret: isize = -3;

  if let Err(e1) = MANAGER.with(|mg| match mg.try_borrow_mut() {
    Ok(mut m) => match m.touch_end() {
      Ok(r) => {
        ret = r;

        Ok(ret)
      }

      Err(e) => {
        return Err(JsValue::from_str(&format!("touch_end failed!: {}", e)));
      }
    },

    Err(e) => {
      return Err(JsValue::from_str(&format!("touch_end failed!: {}", e)));
    }
  }) {
    return Err(JsValue::from_str(&format!("touch_end failed!: {:?}", e1)));
  }

  Ok(ret)
}

/// シングルクリック
///
/// # 引数
/// ## x
/// ## y
///
/// # 戻り値
/// - -3 : 正常終了
/// - -1 : Top選択
/// - 0以上 : セクション選択
/// - それ以外 : 異常終了
///
#[wasm_bindgen]
pub fn single_click(x: i32, y: i32) -> Result<isize, JsValue> {
  //log!("***single_click: x={} y={}", x, y);
  let mut ret: isize = -3;

  if let Err(e1) = MANAGER.with(|mg| match mg.try_borrow_mut() {
    Ok(mut m) => match m.single_click(x, y) {
      Ok(r) => {
        ret = r;

        Ok(ret)
      }

      Err(e) => {
        return Err(JsValue::from_str(&format!("single_click failed!: {}", e)));
      }
    },

    Err(e) => {
      return Err(JsValue::from_str(&format!("single_click failed!: {}", e)));
    }
  }) {
    return Err(JsValue::from_str(&format!(
      "single_click failed!: {:?}",
      e1
    )));
  }

  Ok(ret)
}

/// ダブルクリック
///
/// # 引数
/// ## x
/// ## y
///
/// # 戻り値
/// - -3 : 正常終了
/// - -1 : Top選択
/// - 0以上 : セクション選択
/// - それ以外 : 異常終了
///
#[wasm_bindgen]
pub fn double_click(x: i32, y: i32) -> Result<isize, JsValue> {
  //log!("***double_click: x={} y={}", x, y);
  let mut ret: isize = -3;

  if let Err(e1) = MANAGER.with(|mg| match mg.try_borrow_mut() {
    Ok(mut m) => match m.double_click(x, y) {
      Ok(r) => {
        ret = r;

        Ok(ret)
      }

      Err(e) => {
        return Err(JsValue::from_str(&format!("double_click failed!: {}", e)));
      }
    },

    Err(e) => {
      return Err(JsValue::from_str(&format!("double_click failed!: {}", e)));
    }
  }) {
    return Err(JsValue::from_str(&format!(
      "double_click failed!: {:?}",
      e1
    )));
  }

  Ok(ret)
}

/// 黒塗りモードを変更する
///
/// # 引数
/// ## black
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn mode_change(black: bool) -> Result<(), JsValue> {
  // log!("***mode_change black={}", black);

  if let Err(e1) = MANAGER.with(|mg| match mg.borrow_mut().mode_change(black) {
    Err(e) => {
      return Err(JsValue::from_str(&format!("mode_change failed!: {}", e)));
    }

    _ => Ok(()),
  }) {
    return Err(JsValue::from_str(&format!("mode_change failed!: {:?}", e1)));
  }

  Ok(())
}

/// ツールボタンの操作
///
/// # 引数
///
/// ## step
/// - 1 : 1区切り進む
/// - 2 : 1区切り戻る
/// - 3 : 1単語進む
/// - 4 : 末尾に進む
/// - 5 : 先頭に戻る
/// - 6 : 次の段・節に進む
/// - 7 : 前の段・節に戻る
/// - 9 : 原稿用紙非表示
/// - 10 : 原稿用紙表示
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn tool_func(step: i32) -> Result<isize, JsValue> {
  //log!("***tool_func {}", step);
  let mt: FuncType;
  let mut ret: isize = -3;

  match step {
    1 => mt = FuncType::FdSlash,
    2 => mt = FuncType::BkSlash,
    3 => mt = FuncType::FdOne,
    4 => mt = FuncType::FdBottom,
    5 => mt = FuncType::BkTop,
    6 => mt = FuncType::FdSec,
    7 => mt = FuncType::BkSec,
    9 => mt = FuncType::HideBlock,
    10 => mt = FuncType::ShowBlock,
    _ => {
      return Err(JsValue::from_str(&format!(
        "tool_func invalid step:{}",
        step
      )));
    }
  }

  if let Err(e1) = MANAGER.with(|mg| match mg.try_borrow_mut() {
    Ok(mut m) => match m.tool_func(mt) {
      Ok(r) => {
        ret = r;

        Ok(ret)
      }

      Err(e) => {
        return Err(JsValue::from_str(&format!("tool_func failed!: {}", e)));
      }
    },

    Err(e) => {
      return Err(JsValue::from_str(&format!("tool_func failed!: {}", e)));
    }
  }) {
    return Err(JsValue::from_str(&format!("tool_func failed!: {:?}", e1)));
  }

  Ok(ret)
}

/// 表示/非表示を切り替える
///
/// # 引数
///
/// ## is_hide
/// - 1 : 非表示
/// - 0 : 表示
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn hide(is_hide: i32) -> Result<isize, JsValue> {
  let mut ret: isize = -3;

  if let Err(e1) = MANAGER.with(|mg| match mg.try_borrow_mut() {
    Ok(mut m) => match m.hide(if is_hide == 0 { false } else { true }) {
      Ok(r) => {
        ret = r;

        Ok(ret)
      }

      Err(e) => {
        return Err(JsValue::from_str(&format!("hide failed!: {}", e)));
      }
    },

    Err(e) => {
      return Err(JsValue::from_str(&format!("hide failed!: {}", e)));
    }
  }) {
    return Err(JsValue::from_str(&format!("hide failed!: {:?}", e1)));
  }

  Ok(ret)
}

/// 白板・戻る
///
/// # 引数
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn stroke_back() -> Result<(), JsValue> {
  //log!("***stroke_back");

  if let Err(e1) = MANAGER.with(|mg| match mg.borrow_mut().stroke_back() {
    Err(e) => {
      return Err(JsValue::from_str(&format!("stroke_back failed!: {}", e)));
    }

    _ => Ok(()),
  }) {
    return Err(JsValue::from_str(&format!("stroke_back failed!: {:?}", e1)));
  }

  Ok(())
}

/// 白板・消去
///
/// # 引数
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn stroke_clear() -> Result<(), JsValue> {
  //log!("***stroke_clear");

  if let Err(e1) = MANAGER.with(|mg| match mg.borrow_mut().stroke_clear() {
    Err(e) => {
      return Err(JsValue::from_str(&format!("stroke_clear failed!: {}", e)));
    }

    _ => Ok(()),
  }) {
    return Err(JsValue::from_str(&format!(
      "stroke_clear failed!: {:?}",
      e1
    )));
  }

  Ok(())
}
