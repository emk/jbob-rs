//! Runtime for the J-Bob proof language, which is basically a tiny subset of
//! Scheme.

#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod types;
mod read;

use self::read::read_str;

/// Our grammar, generated automatically from `grammar.rustpeg` by our
/// `build.rs` script using `rustpeg`.
mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

/// Public entry point.
#[wasm_bindgen]
pub fn eval_str(name: &str) -> String {
    let value = read_str(name).unwrap();
    format!("{}", value)
}
