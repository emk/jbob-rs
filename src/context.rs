//! Interpreter context.

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};
use wasm_bindgen::prelude::*;

use ast::Ast;
use errors::Error;
use eval::eval_ast;
use environment::Environment;
use functions::add_prelude_functions;
use read::read_file;
use types::{Symbol, Value};

/// A context represents the internal state of a single-threaded Scheme
/// interpreter.
#[wasm_bindgen]
pub struct Context {
    /// All the unique symbols known to our interpreter.
    symbols: HashMap<String, Symbol>,
    /// Global environment.
    global_environment: Rc<RefCell<Environment>>,
}

impl Context {
    /// Given a string, find the corresponding unique symbol.
    pub fn intern_symbol(&mut self, s: &str) -> Symbol {
        self.symbols
            .entry(s.to_owned())
            .or_insert_with(|| Symbol(Rc::new(s.to_owned())))
            .to_owned()
    }

    /// Given a string, find the corresponding unique symbol as a Scheme value.
    pub fn intern(&mut self, s: &str) -> Value {
        Value::Symbol(self.intern_symbol(s))
    }

    /// Get the `nil` symbol associated with this context.
    pub fn nil(&mut self) -> Value {
        // TODO: Cache this at startup.
        self.intern("nil")
    }

    /// Get the `t` symbol associated with this context.
    pub fn t(&mut self) -> Value {
        // TODO: Cache this at startup.
        self.intern("t")
    }

    /// Construct a Scheme Boolean value from a Rust value.
    pub fn bool_value(&mut self, b: bool) -> Value {
        if b {
            self.t()
        } else {
            self.nil()
        }
    }

    /// Get the global environment associated with this interpreter context.
    pub fn global_environment(&self) -> Rc<RefCell<Environment>> {
        self.global_environment.clone()
    }

    /// Evaluate a file containing a series of s-expressions, and return the
    /// result of the final value in the file.
    pub fn eval_file(&mut self, input: &str) -> Result<Value, Error> {
        let env = self.global_environment();
        let values = read_file(self, input)?;
        let mut last_value = Value::Null;
        for value in &values {
            let ast = Ast::build(self, value)?;
            last_value = eval_ast(self, &env, &ast)?;
        }
        Ok(last_value)
    }
}

impl Default for Context {
    fn default() -> Context {
        let mut ctx = Context {
            symbols: Default::default(),
            global_environment: Default::default(),
        };
        add_prelude_functions(&mut ctx);
        ctx
    }
}
