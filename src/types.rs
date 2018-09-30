//! Fundamental types for our interpreter.

use std::{
    collections::HashMap,
    fmt,
    rc::Rc,
};

/// A context represents the internal state of a single-threaded Scheme
/// interpreter.
#[derive(Default)]
pub struct Context {
    /// All the unique symbols known to our interpreter.
    symbols: HashMap<String, Value>,
}

impl Context {
    /// Given a string, find the corresponding unique symbol value.
    pub fn intern(&mut self, s: &str) -> Value {
        self.symbols
            .entry(s.to_owned())
            .or_insert_with(|| Value::Symbol(Rc::new(s.to_owned())))
            .to_owned()
    }
}

/// A Scheme value.
#[derive(Clone, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum Value {
    /// We store integers as immediate values, without wrapping them in an RC,
    /// because they're cheap to copy.
    Integer(i64),
    /// A symbol is a unique, tokenized string. These should only be created
    /// using `Context::intern`
    Symbol(Rc<String>),
    /// The empty list is a special value in Scheme.
    Null,
    /// We represent a cons cell as a pair stored on the heap.
    Cons(Rc<(Value, Value)>),
}

impl Value {
    /// Constuct a cons cell from two values.
    pub fn cons(car: Value, cdr: Value) -> Self {
        Value::Cons(Rc::new((car, cdr)))
    }

    /// Wrap a value `val` as `'(quote val)`.
    pub fn quote(ctx: &mut Context, value: Value) -> Self {
        Value::cons(
            ctx.intern("quote"),
            Value::cons(value, Value::Null),
        )
    }
}

impl PartialEq for Value {
    fn eq(&self, rhs: &Value) -> bool {
        match (self, rhs) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            // Symbols are equal if they point to the exact same interned
            // string. We never do a string comparision.
            (Value::Symbol(a), Value::Symbol(b)) => Rc::ptr_eq(a, b),
            (Value::Null, Value::Null) => true,
            (Value::Cons(a), Value::Cons(b)) => {
                // Optimization: Two `Cons` cells are eqyal if they point
                // to the same underlying pair, or if they have equal contents.
                Rc::ptr_eq(a, b) || a == b
            }
            (_, _) => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Null => write!(f, "()"),
            Value::Cons(c) => {
                write!(f, "({}", c.as_ref().0)?;
                let mut cdr = &c.as_ref().1;
                while let Value::Cons(next) = cdr {
                    write!(f, " {}", next.as_ref().0)?;
                    cdr = &next.as_ref().1;
                }
                match cdr {
                    Value::Null => {}
                    other => { write!(f, " . {}", other)?; }
                }
                ")".fmt(f)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_eq() {
        let mut ctx = Context::default();
        assert_eq!(Value::Null, Value::Null);
        assert_eq!(ctx.intern("a"), ctx.intern("a"));
    }

    #[test]
    fn quote_value() {
        let mut ctx = Context::default();
        assert_eq!(
            Value::quote(&mut ctx, Value::Integer(1)),
            Value::cons(
                ctx.intern("quote"),
                Value::cons(Value::Integer(1), Value::Null),
            ),
        );
    }

    #[test]
    fn display_value() {
        let mut ctx = Context::default();
        let value = Value::cons(
            Value::cons(Value::Integer(1), ctx.intern("a")),
            Value::cons(Value::Null, Value::Null),
        );
        assert_eq!(format!("{}", value), "((1 . a) ())");
    }
}
