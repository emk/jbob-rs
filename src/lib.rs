//! Runtime for the J-Bob proof language, which is basically a tiny subset of
//! Scheme.

#![warn(missing_docs)]

extern crate wasm_bindgen;

use std::ops::Deref;
use wasm_bindgen::prelude::*;

mod types;
mod read;

use self::types::{Context, Value};

/// Our grammar, generated automatically from `grammar.rustpeg` by our
/// `build.rs` script using `rustpeg`.
mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

/// Create a new Scheme execution context.
#[wasm_bindgen]
pub fn new_context() -> Context {
    Context::default()
}

/// `#[wasm_bindgen]` can't pass Rust-style `enum` values to JavaScript, so
/// just put a trivial wrapper around it.
#[wasm_bindgen]
pub struct Wrapped(Value);

impl Deref for Wrapped {
    type Target = Value;

    fn deref(&self) -> &Value {
        &self.0
    }
}

/// Parse a single Scheme expression.
#[wasm_bindgen]
pub fn read_sexpr(
    ctx: &mut Context,
    text: &str,
) -> Result<Wrapped, JsValue> {
    Ok(Wrapped(read::read_sexpr(ctx, text)?))
}

/// Public entry point.
#[wasm_bindgen]
pub fn print(_ctx: &mut Context, value: &Wrapped) -> String {
    format!("{}", value.deref())
}
