/* tslint:disable */
/* eslint-disable */
/**
* 起動時処理
*/
export function main(): void;
/**
* 疎通確認
*
* # 引数
* input
*
* # 戻り値
* - input + 1
* @param {number} input
* @returns {number}
*/
export function ping(input: number): number;
/**
* Googleフォントロード処理
*
* # 引数
* なし
*
* # 戻り値
* なし
*/
export function load_font(): void;
/**
* 文書をセットする
*
* # 引数
* ## id
* ## title
* ## vertical
* - 1 : 横書き
* - 2 : 縦書き
* ## font_size
* ## current
*
* # 戻り値
* なし
* @param {number} id
* @param {string} title
* @param {number} vertical
* @param {number} font_size
* @param {number} current
*/
export function set_doc(id: number, title: string, vertical: number, font_size: number, current: number): void;
/**
* 文書の行をセットする
*
* # 引数
* ## seq
* ## text
*
* # 戻り値
* なし
* @param {number} seq
* @param {string} text
*/
export function set_source(seq: number, text: string): void;
/**
* 文書ツリーを生成する
*
* # 引数
* なし
*
* # 戻り値
* なし
*/
export function build_tree(): void;
/**
* 文書を表示する
*
* # 引数
* ## width
* - キャンバスの幅
* ## height
* - キャンバスの高さ
* ## is_dark
* - true: ダークモード
* ## is_android
* - true: Android
*
* # 戻り値
* なし
* @param {number} width
* @param {number} height
* @param {boolean} is_dark
* @param {boolean} is_android
*/
export function draw_doc(width: number, height: number, is_dark: boolean, is_android: boolean): void;
/**
* キャンバスサイズ変更
*
* # 引数
* ## width
* - キャンバスの幅
* ## height
* - キャンバスの高さ
* ## is_dark
* - true: ダークモード
*
* # 戻り値
* なし
* @param {number} width
* @param {number} height
* @param {boolean} is_dark
*/
export function resize(width: number, height: number, is_dark: boolean): void;
/**
* タブを切り替える
*
* # 引数
* ## tab
* - 0: 本文
* - 1: 目次
* - 2: Box
* - 3: 白板
*
* # 戻り値
* なし
* @param {number} tab
* @param {number} width
* @param {number} height
* @param {boolean} is_dark
* @returns {number}
*/
export function tab_change(tab: number, width: number, height: number, is_dark: boolean): number;
/**
* 現在のセクションを返す
*
* # 引数
* なし
*
* # 戻り値
* セクション
* @returns {number}
*/
export function get_section(): number;
/**
* タッチ開始
*
* # 引数
* ## x
* ## y
*
* # 戻り値
* なし
* @param {number} x
* @param {number} y
*/
export function touch_start(x: number, y: number): void;
/**
* タッチを移動する
*
* # 引数
* ## x
* ## y
*
* # 戻り値
* なし
* @param {number} x
* @param {number} y
*/
export function touch_move(x: number, y: number): void;
/**
* タッチ終了
*
* # 引数
* なし
*
* # 戻り値
* - -2 : 正常終了
* - -1 : Top選択
* - 0以上 : セクション選択
* - それ以外 : 異常終了
* @returns {number}
*/
export function touch_end(): number;
/**
* 黒塗りモードを変更する
*
* # 引数
* ## black
*
* # 戻り値
* なし
* @param {boolean} black
*/
export function mode_change(black: boolean): void;
/**
* ツールボタンの操作
*
* # 引数
*
* ## step
* - 1 : 1区切り進む
* - 2 : 1区切り戻る
* - 3 : 1単語進む
* - 4 : 末尾に進む
* - 5 : 先頭に戻る
* - 6 : 次の段・節に進む
* - 7 : 前の段・節に戻る
*
* # 戻り値
* なし
* @param {number} step
*/
export function tool_func(step: number): void;
/**
* 白板・戻る
*
* # 引数
*
* # 戻り値
* なし
*/
export function stroke_back(): void;
/**
* 白板・消去
*
* # 引数
*
* # 戻り値
* なし
*/
export function stroke_clear(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: () => void;
  readonly ping: (a: number) => number;
  readonly load_font: () => void;
  readonly set_doc: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly set_source: (a: number, b: number, c: number) => void;
  readonly build_tree: () => void;
  readonly draw_doc: (a: number, b: number, c: number, d: number) => void;
  readonly resize: (a: number, b: number, c: number) => void;
  readonly tab_change: (a: number, b: number, c: number, d: number) => number;
  readonly get_section: () => number;
  readonly touch_start: (a: number, b: number) => void;
  readonly touch_move: (a: number, b: number) => void;
  readonly touch_end: () => number;
  readonly mode_change: (a: number) => void;
  readonly tool_func: (a: number) => void;
  readonly stroke_back: () => void;
  readonly stroke_clear: () => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6a3f6fb25b123bd8: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf6b2e8a9d37b6d3b: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
