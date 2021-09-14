#[macro_use]
mod util;

mod click;
mod manager;
mod model;
mod view;

use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::FontFace;

// 黒塗り移動
pub enum MoveType {
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
}

pub enum TabType {
  TabText,
  TabContents,
  TabBoard,
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

pub fn click_handle() -> click::ClickHandle {
  let cb = Closure::wrap(Box::new(|| {
    //log!("timeout elapsed!");
    MANAGER.with(|mg| match mg.try_borrow_mut() {
      Ok(mut m) => match m.click() {
        Ok(t) => t,

        Err(_) => 1,
      },

      Err(_) => 1,
    });
  }) as Box<dyn FnMut()>);

  // 600ミリ秒後にクロージャーにラップされたクリックイベントを発生させる。
  // ダブルクリックが発生していなければシングルクリック処理を行う。
  let id = setTimeout(&cb, 600);

  click::ClickHandle {
    id: id,
    closure: cb,
  }
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
  //log!("***ping");
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
      .set_doc(id, title, vertical, font_size, current)
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
/// - false: 本文
/// - true: 目次
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn tab_change(tab: i32, width: i32, height: i32, is_dark: bool) -> Result<(), JsValue> {
  //log!("***tab_change tab={}", tab);
  let t: TabType;
  match tab {
    1 => t = TabType::TabContents,
    2 => t = TabType::TabBoard,
    _ => t = TabType::TabText,
  }

  if let Err(e1) = MANAGER.with(
    |mg| match mg.borrow_mut().tab_change(t, width, height, is_dark) {
      Err(e) => {
        return Err(JsValue::from_str(&format!("tab_change failed!: {}", e)));
      }

      _ => Ok(()),
    },
  ) {
    return Err(JsValue::from_str(&format!("tab_change failed!: {:?}", e1)));
  }

  Ok(())
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
  //log!("***mode_change black={}", black);

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

/// 黒塗りを移動する
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
///
/// # 戻り値
/// なし
///
#[wasm_bindgen]
pub fn black_step(step: i32) -> Result<(), JsValue> {
  //log!("***black_step {}", step);
  let mt: MoveType;

  match step {
    1 => mt = MoveType::FdSlash,
    2 => mt = MoveType::BkSlash,
    3 => mt = MoveType::FdOne,
    4 => mt = MoveType::FdBottom,
    5 => mt = MoveType::BkTop,
    6 => mt = MoveType::FdSec,
    7 => mt = MoveType::BkSec,
    _ => {
      return Err(JsValue::from_str(&format!(
        "black_step invalid step:{}",
        step
      )));
    }
  }

  if let Err(e1) = MANAGER.with(|mg| match mg.borrow_mut().black_step(mt) {
    Err(e) => {
      return Err(JsValue::from_str(&format!("black_step failed!: {}", e)));
    }

    _ => Ok(()),
  }) {
    return Err(JsValue::from_str(&format!("black_step failed!: {:?}", e1)));
  }

  Ok(())
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
