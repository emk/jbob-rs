//! Environments map variables to values.

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use types::{Symbol, Value};

/// A mapping between variables and values.
#[derive(Default)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    bindings: HashMap<Symbol, Value>,
}

impl Environment {
    pub fn make_child(parent: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            parent: Some(parent),
            bindings: HashMap::default(),
        }))
    }

    pub fn define(&mut self, symbol: Symbol, value: Value) {
        self.bindings.insert(symbol, value);
    }

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
