//! Environments map variables to values.

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use types::{Symbol, Value};

/// A mapping between variables and values.
#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    bindings: HashMap<Symbol, Value>,
}

impl Environment {
    /// Consturct a fresh "child" environment, which inherits all the
    /// definitions in the parent environment, but allows new, local definitions
    /// to be added.
    pub fn make_child(parent: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            parent: Some(parent),
            bindings: HashMap::default(),
        }))
    }

    /// Define a new binding in this environment.
    pub fn define(&mut self, symbol: Symbol, value: Value) {
        self.bindings.insert(symbol, value);
    }

    /// Look up a binding in either this environment, or in the first ancestor
    /// environment which contains it.
    pub fn lookup(&self, symbol: &Symbol) -> Option<Value> {
        if let Some(local_value) = self.bindings.get(symbol) {
            Some(local_value.to_owned())
        } else if let Some(parent) = self.parent.as_ref() {
            parent.borrow().lookup(symbol)
        } else {
            None
        }
    }
}
