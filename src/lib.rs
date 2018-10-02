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

/// Evaluate a string as Scheme source code, updating `ctx` accordingly and
/// returning the final value.
#[wasm_bindgen]
pub fn eval_file(
    ctx: &mut Context,
    input: &str,
) -> Result<Wrapped, JsValue> {
    Ok(Wrapped(ctx.eval_file(input)?))
}

/// Public entry point.
#[wasm_bindgen]
pub fn print(_ctx: &mut Context, value: &Wrapped) -> String {
    format!("{}", value.deref())
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_eval_eq {
        ($ctx:expr, $source1:expr, $source2:expr) => {
            {
                let ctx: &mut Context = $ctx;
                let left = ctx.eval_file($source1).unwrap();
                let right = ctx.eval_file($source2).unwrap();
                println!("left: {}", left);
                println!("right: {}", right);
                assert_eq!(left, right);
            }
        };
    }

    #[test]
    fn primitives() {
        let mut ctx = Context::default();
        assert_eval_eq!(&mut ctx, "(atom '())", "'t");
        assert_eval_eq!(&mut ctx, "(atom '(1))", "'nil");
        assert_eval_eq!(&mut ctx, "(car (cons 1 2))", "1");
        assert_eval_eq!(&mut ctx, "(cdr (cons 1 2))", "2");
        assert_eval_eq!(&mut ctx, "(if 't 1 2)", "1");
        assert_eval_eq!(&mut ctx, "(if 'nil 1 2)", "2");
        assert_eval_eq!(&mut ctx, "(natp '())", "'nil");
        assert_eval_eq!(&mut ctx, "(natp -1)", "'nil");
        assert_eval_eq!(&mut ctx, "(natp 0)", "'t");
        assert_eval_eq!(&mut ctx, "(natp 1)", "'t");
        assert_eval_eq!(&mut ctx, "(equal '() '())", "'t");
        assert_eval_eq!(&mut ctx, "(equal '(1) '(1))", "'t");
        assert_eval_eq!(&mut ctx, "(equal '(1) '(2))", "'nil");
        assert_eval_eq!(&mut ctx, "(equal 'a 'a)", "'t");
        assert_eval_eq!(&mut ctx, "(equal 'a 'b)", "'nil");
        assert_eval_eq!(&mut ctx, "(equal 1 1)", "'t");
        assert_eval_eq!(&mut ctx, "(equal 1 2)", "'nil");
    }

    #[test]
    fn recursion() {
        let mut ctx = Context::default();
        let input = "
        (defun my-len (xs)
          (if (atom xs)
            0
            (+ 1 (my-len (cdr xs)))))
        (my-len '(1 2 3 4))
        ";
        assert_eval_eq!(&mut ctx, input, "4");
    }

    #[test]
    fn jbob() {
        let mut ctx = Context::default();
        ctx.eval_file(include_str!("scheme/j-bob.scm")).unwrap();
    }

    #[test]
    fn little_prover_quick() {
        let mut ctx = Context::default();
        ctx.eval_file(include_str!("scheme/j-bob.scm")).unwrap();
        ctx.eval_file(include_str!("scheme/little-prover.scm")).unwrap();
        assert_eval_eq!(&mut ctx, "(chapter1.example1)", "''ham");
    }

    #[test]
    #[ignore]
    fn little_prover_align_align() {
        let mut ctx = Context::default();
        ctx.eval_file(include_str!("scheme/j-bob.scm")).unwrap();
        ctx.eval_file(include_str!("scheme/little-prover.scm")).unwrap();
        let expected = include_str!("scheme/align-align-result.scm");
        assert_eval_eq!(
            &mut ctx,
            "(dethm.align/align)",
            expected
        );
    }
}
