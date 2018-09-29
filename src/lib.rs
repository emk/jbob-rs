extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let mut msg = "Hello, ".to_owned();
    msg.push_str(name);
    msg.push_str("!");
    alert(&msg);
}
