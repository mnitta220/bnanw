use super::area;
use super::canvas;

pub trait Panel {
  fn new() -> Self;

  /// 文書を表示する。
  fn draw(
    &mut self,
    cv: &canvas::Canvas,
    areas: &mut Vec<area::Area>,
    is_black: bool,
    is_dark: bool,
    is_hide: bool,
  ) -> Result<isize, &'static str>;

  /// タッチ開始
  fn touch_start(&mut self, x: i32, y: i32) -> Result<(), &'static str>;

  /// タッチを移動する
  fn touch_move(&mut self, x: i32, y: i32) -> Result<isize, &'static str>;

  /// タッチ終了
  ///
  /// # 戻り値
  /// - -3 : 正常終了
  /// - -2 : ダブルタップ
  /// - -1 : Top選択
  /// - 0以上 : セクション選択
  /// - それ以外 : 異常終了
  ///
  fn touch_end(&mut self) -> Result<isize, &'static str>;
}
