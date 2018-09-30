//! Fundamental types for our interpreter.

use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};

lazy_static! {
    /// Our table of interned symbols.
    static ref SYMBOLS: Mutex<HashMap<String, Arc<String>>> =
        Mutex::new(HashMap::new());
}

/// A symbol is a unique, tokenized string. We represent them as a ref-counted
/// string.
#[derive(Eq)]
#[cfg_attr(test, derive(Debug))]
pub struct Symbol(Arc<String>);

impl Symbol {
    /// Given a string, find the unique symbol corresponding to that string.
    pub fn intern<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        let s = s.into();
        let mut symbols = SYMBOLS.lock().unwrap();
        let symbol = symbols
            .entry(s.clone())
            .or_insert_with(|| Arc::new(s))
            .to_owned();
        Symbol(symbol)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.as_ref().fmt(f)
    }
}

impl PartialEq for Symbol {
    /// Two symbols are equal if they point to the same interned string.
    fn eq(&self, rhs: &Symbol) -> bool {
        Arc::ptr_eq(&self.0, &rhs.0)
    }
}

/// A Scheme value.
#[derive(Eq, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum Value {
    /// We store integers as immediate values, without wrapping them in an RC,
    /// because they're cheap to copy.
    Integer(i64),
    /// Symbols are represented as strings, but they should only be created
    /// using `Value::symbol("my-string")`. Only one copy of each symbol
    /// should exist.
    Symbol(Symbol),
    /// The empty list is a special value in Scheme.
    Null,
    /// We represent a cons cell as a pair stored on the heap.
    Cons(Arc<(Value, Value)>),
}

impl Value {
    /// Construct a symbol from a string.
    pub fn symbol<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Value::Symbol(Symbol::intern(s))
    }

    /// Constuct a cons cell from two values.
    pub fn cons(car: Value, cdr: Value) -> Self {
        Value::Cons(Arc::new((car, cdr)))
    }

    /// Wrap a value `val` as `'(quote val)`.
    pub fn quote(value: Value) -> Self {
        Value::cons(
            Value::symbol("quote"),
            Value::cons(value, Value::Null),
        )
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
        assert_eq!(Value::Null, Value::Null);
        assert_eq!(Value::symbol("a"), Value::symbol("a"));
    }

    #[test]
    fn quote_value() {
        assert_eq!(
            Value::quote(Value::Integer(1)),
            Value::cons(
                Value::symbol("quote"),
                Value::cons(Value::Integer(1), Value::Null),
            ),
        );
    }

    #[test]
    fn display_value() {
        let value = Value::cons(
            Value::cons(Value::Integer(1), Value::symbol("a")),
            Value::cons(Value::Null, Value::Null),
        );
        assert_eq!(format!("{}", value), "((1 . a) ())");
    }
}
