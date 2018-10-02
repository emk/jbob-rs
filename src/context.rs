//! Interpreter context.

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};
use wasm_bindgen::prelude::*;

use environment::Environment;
use types::{Symbol, Value};

/// A context represents the internal state of a single-threaded Scheme
/// interpreter.
#[wasm_bindgen]
#[derive(Default)]
pub struct Context {
    /// All the unique symbols known to our interpreter.
    symbols: HashMap<String, Value>,
    /// Global environment.
    global_environment: Rc<RefCell<Environment>>,
}

impl Context {
    /// Given a string, find the corresponding unique symbol value.
    pub fn intern(&mut self, s: &str) -> Value {
        self.symbols
            .entry(s.to_owned())
            .or_insert_with(|| Value::Symbol(Symbol(Rc::new(s.to_owned()))))
            .to_owned()
    }

    /// Get the `nil` symbol associated with this context.
    pub fn nil(&mut self) -> Value {
        // TODO: Cache this at startup.
        self.intern("nil")
    }

    /// Get the global environment associated with this interpreter context.
    pub fn global_environment(&self) -> Rc<RefCell<Environment>> {
        self.global_environment.clone()
    }
}
