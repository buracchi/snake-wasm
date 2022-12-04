use wasm_bindgen::prelude::*;

mod snake;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {} from Rust!", name));
}
