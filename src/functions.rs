//! Callable Scheme functions.

use std::fmt;

use context::Context;
use errors::Error;
use types::{Symbol, Value};

/// A Scheme function.
pub struct Function {
    name: Option<Symbol>,
    arity: usize,
    func: Box<Fn(&mut Context, &[Value]) -> Result<Value, Error>>,
}

impl Function {
    /// Create a new function from a Rust closure.
    pub fn new<F>(
        name: Option<Symbol>,
        arity: usize,
        func: F,
    ) -> Function
    where
        F: Fn(&mut Context, &[Value]) -> Result<Value, Error> + 'static,
    {
        Function { name, arity, func: Box::new(func) }
    }

    /// Call this function with the specified arguments.
    pub fn call(
        &self,
        ctx: &mut Context,
        arguments: &[Value],
    ) -> Result<Value, Error> {
        if arguments.len() != self.arity {
            Err(format!(
                "called '{}' with {} arguments, but it expected {}",
                self, arguments.len(), self.arity,
            ).into())
        } else {
            (self.func)(ctx, arguments)
        }
    }
}

#[cfg(test)]
impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "#<function '{}'>", name)
        } else {
            write!(f, "#<function>")
        }
    }
}
