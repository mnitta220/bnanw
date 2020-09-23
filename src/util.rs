use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

pub fn log_str(s: &str) {
  console::log_1(&JsValue::from_str(s));
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (crate::util::log_str(&format!($($arg)*)));
}

pub fn get_document() -> Result<web_sys::Document, &'static str> {
  let window: web_sys::Window;

  match web_sys::window() {
    Some(w) => {
      window = w;
    }

    _ => {
      return Err("get_document: web_sys::window() failed!");
    }
  }

  match window.document() {
    Some(d) => Ok(d),

    _ => {
      return Err("get_document: web_sys::document() failed!");
    }
  }
}

pub fn get_canvas(id: &str) -> Result<web_sys::HtmlCanvasElement, &'static str> {
  let document: web_sys::Document;
  let canvas: web_sys::Element;

  document = get_document()?;

  match document.get_element_by_id(id) {
    Some(e) => {
      canvas = e;
    }

    _ => {
      return Err("get_canvas: document.get_element_by_id('ca1') failed!");
    }
  }

  let canvas: web_sys::HtmlCanvasElement = canvas
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_| ())
    .unwrap();

  Ok(canvas)
}

pub fn get_context(
  canvas: &web_sys::HtmlCanvasElement,
) -> Result<web_sys::CanvasRenderingContext2d, &'static str> {
  let ct: web_sys::CanvasRenderingContext2d;
  match canvas
    .get_context("2d")
    .unwrap()
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
  {
    Ok(c) => {
      ct = c;
      Ok(ct)
    }

    _ => Err("get_context failed!"),
  }
}
