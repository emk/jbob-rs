//! Runtime for the J-Bob proof language, which is basically a tiny subset of
//! Scheme.

#![warn(missing_docs)]

extern crate wasm_bindgen;

use std::ops::Deref;
use wasm_bindgen::prelude::*;

mod ast;
mod context;
mod environment;
mod errors;
mod eval;
mod functions;
mod read;
mod types;

use context::Context;
use types::Value;
use ast::Ast;

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

/// Evaluate a string as Scheme source code, updating `ctx` accordingly and
/// returning the final value.
#[wasm_bindgen]
pub fn eval_file(
    ctx: &mut Context,
    input: &str,
) -> Result<Wrapped, JsValue> {
    let values = read::read_file(ctx, input)?;
    for value in values {
        Ast::build(ctx, &value)?;
    }
    Ok(Wrapped(Value::Null))
}

/// Public entry point.
#[wasm_bindgen]
pub fn print(_ctx: &mut Context, value: &Wrapped) -> String {
    format!("{}", value.deref())
}

#[cfg(test)]
mod test {
    use super::*;
    use read::read_file;
    use ast::Ast;

    fn check_source(source: &str) {
        let mut ctx = Context::default();
        let values = read_file(&mut ctx, source).unwrap();
        for value in &values {
            Ast::build(&mut ctx, value).unwrap();
        }
    }

    #[test]
    fn check_jbob_source() {
        check_source(include_str!("scheme/j-bob.scm"));
    }

    #[test]
    fn check_little_prover_source() {
        check_source(include_str!("scheme/little-prover.scm"));
    }
}
