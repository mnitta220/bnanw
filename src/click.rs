use wasm_bindgen::prelude::*;

pub struct ClickHandle {
  pub id: i32,
  pub closure: Closure<dyn FnMut()>,
}
