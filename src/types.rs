//! Fundamental types for our interpreter.

use std::{
    hash::{Hash, Hasher},
    fmt,
    rc::Rc,
};

use context::Context;
use errors::Error;
use functions::Function;

/// A symbol is a unique string that can be tested for equality using pointer
/// comparison. It should only be created using `Context::intern`.
#[derive(Clone, Eq)]
#[cfg_attr(test, derive(Debug))]
pub struct Symbol(pub Rc<String>);

impl Symbol {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the pointer, not the string. This is OK because symbols use
        // pointer equality.
        let ptr = &self.0.as_ref().as_bytes()[0] as *const u8;
        ptr.hash(state)
    }
}

impl PartialEq for Symbol {
    fn eq(&self, rhs: &Symbol) -> bool {
        Rc::ptr_eq(&self.0, &rhs.0)
    }
}

/// A Scheme value.
#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub enum Value {
    /// We store integers as immediate values, without wrapping them in an RC,
    /// because they're cheap to copy.
    Integer(i64),
    /// A symbol is a unique, tokenized string. These should only be created
    /// using `Context::intern`
    Symbol(Symbol),
    /// The empty list is a special value in Scheme.
    Null,
    /// We represent a cons cell as a pair stored on the heap.
    Cons(Rc<(Value, Value)>),
    /// A Scheme function.
    Function(Rc<Function>),
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

    /// Return an iterator over a Scheme list.
    pub fn iter(&self) -> ValueIter {
        ValueIter { remaining: self.to_owned() }
    }

    /// If this value is a symbol, return it. Otherwise return an error.
    pub fn expect_symbol(&self) -> Result<Symbol, Error> {
        if let Value::Symbol(symbol) = self {
            Ok(symbol.to_owned())
        } else {
            Err(format!("expected symbol, found {}", self).into())
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, rhs: &Value) -> bool {
        match (self, rhs) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Cons(a), Value::Cons(b)) => {
                // Optimization: Two `Cons` cells are equal if they point to the
                // same underlying pair, or if they have equal contents.
                Rc::ptr_eq(a, b) || a == b
            }
            (Value::Function(a), Value::Function(b)) => Rc::ptr_eq(&a, &b),
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
                write!(f, ")")
            }
            Value::Function(func) => write!(f, "{}", func),
        }
    }
}

/// Iterator over Scheme lists. Will return an error if the final `cdr` is not
/// `Value::Null`.
pub struct ValueIter {
    remaining: Value,
}

impl Iterator for ValueIter {
    type Item = Result<Value, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.remaining.clone() {
            Value::Null => None,
            Value::Cons(c) => {
                let (car, cdr) = c.as_ref().to_owned();
                self.remaining = cdr;
                Some(Ok(car))
            }
            value => {
                let msg = format!("unexpected value in cdr: {}", value);
                Some(Err(msg.into()))
            }
        }
    }
}

#[cfg(test)]
mod test {
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
