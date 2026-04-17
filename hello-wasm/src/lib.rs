use wasm_bindgen::prelude::*;

// Импортируем функцию alert из JavaScript
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Экспортируем функцию greet в JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
