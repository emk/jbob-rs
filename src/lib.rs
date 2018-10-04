//! Runtime for the J-Bob proof language, which is basically a tiny subset of
//! Scheme.

#![warn(missing_docs)]

extern crate wasm_bindgen;

use std::ops::{Deref, DerefMut};
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

/// The source code to the J-Bob proof assistant.
const JBOB_SOURCE: &str = include_str!("scheme/j-bob.scm");
// The source code to the Little Prover book.
const LITTLE_PROVER_SOURCE: &str = include_str!("scheme/little-prover.scm");

/// Our grammar, generated automatically from `grammar.rustpeg` by our
/// `build.rs` script using `rustpeg`.
mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

/// `#[wasm_bindgen]` can't pass Rust-style `enum` values to JavaScript, so
/// just put a trivial wrapper around it.
#[wasm_bindgen]
pub struct JBobValue(Value);

#[wasm_bindgen]
impl JBobValue {
    /// Format this value as a string.
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{}", self.deref())
    }
}

impl Deref for JBobValue {
    type Target = Value;

    fn deref(&self) -> &Value {
        &self.0
    }
}

/// The interpreter context for a single threaded interpreter. This contains
/// global definitions, etc.
#[wasm_bindgen]
#[derive(Default)]
pub struct JBobContext(Context);

impl Deref for JBobContext {
    type Target = Context;

    fn deref(&self) -> &Context {
        &self.0
    }
}

impl DerefMut for JBobContext {
    fn deref_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}

#[wasm_bindgen]
impl JBobContext {
    /// Create a new `JBobContext`.
    ///
    /// `#[wasm_bindgen(constructor)]` is disabled pending
    /// https://github.com/rustwasm/wasm-bindgen/issues/917.
    pub fn new() -> JBobContext {
        Self::default()
    }

    /// Load the J-Bob source code into the context.
    #[wasm_bindgen(js_name = requireJBob)]
    pub fn require_jbob(&mut self) -> Result<(), JsValue> {
        self.eval_file(JBOB_SOURCE)?;
        Ok(())
    }

    /// Load the source code from _The Little Prover_ into the context.
    #[wasm_bindgen(js_name = requireLittleProver)]
    pub fn require_little_prover(&mut self) -> Result<(), JsValue> {
        self.eval_file(LITTLE_PROVER_SOURCE)?;
        Ok(())
    }

    /// Is the supplied string a valid s-expression?
    #[wasm_bindgen(js_name = isValidSExpr)]
    pub fn is_valid_sexpr(&mut self, input: &str) -> bool {
        read::read_sexpr(&mut self.0, input).is_ok()
    }

    /// Evaluate a string as Scheme source code, updating `ctx` accordingly and
    /// returning the final value.
    pub fn eval(&mut self, input: &str) -> Result<JBobValue, JsValue> {
        Ok(JBobValue(self.eval_file(input)?))
    }
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
    fn recursion_size() {
        let mut ctx = Context::default();
        assert_eval_eq!(&mut ctx, "(if (atom '(1 2)) '0 (+ '1 (+ (size (car '(1 2))) (size (cdr '(1 2))))))", "2");
        assert_eval_eq!(&mut ctx, "(size '(1 2))", "2");
    }

    #[test]
    fn jbob() {
        let mut ctx = JBobContext::default();
        ctx.require_jbob().unwrap();
    }

    #[test]
    fn little_prover_quick() {
        let mut ctx = JBobContext::default();
        ctx.require_jbob().unwrap();
        ctx.require_little_prover().unwrap();
        assert_eval_eq!(&mut ctx, "(chapter1.example1)", "''ham");
    }

    #[test]
    #[ignore]
    fn little_prover_align_align() {
        let mut ctx = JBobContext::default();
        ctx.require_jbob().unwrap();
        ctx.require_little_prover().unwrap();
        let expected = include_str!("scheme/align-align-result.scm");
        assert_eval_eq!(&mut ctx, "(dethm.align/align)", expected);
    }
}
