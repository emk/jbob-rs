//! Abstract syntax trees. We use these when interpreting code.

use std::{fmt, rc::Rc};

use context::Context;
use errors::Error;
use types::{Symbol, Value};

/// Names of built-in special forms.
#[derive(Clone, Copy)]
enum SpecialForm {
    Quote,
    If,
    Defun,
    Dethm,
}

impl SpecialForm {
    /// If `value` is a symbol naming a special form, return that special form.
    /// Otherwise, return `None`.
    fn from_value(value: &Value) -> Option<SpecialForm> {
        if let Value::Symbol(symbol) = value {
            // TODO: We might want to cache these symbols in `ctx` to speed this
            // up and to allow the use of "uninterned" symbols at some point.
            match symbol.as_str() {
                "quote" => return Some(SpecialForm::Quote),
                "if" => return Some(SpecialForm::If),
                "defun" => return Some(SpecialForm::Defun),
                "dethm" => return Some(SpecialForm::Dethm),
                _ => {}
            }
        }
        None
    }

    /// How many arguments should this special form have?
    fn arity(self) -> usize {
        match self {
            SpecialForm::Quote => 1,
            SpecialForm::If => 3,
            SpecialForm::Defun | SpecialForm::Dethm => 3,
        }
    }

    /// Ensure that our actual arity matches our expected arity.
    fn check_arity(self, actual: usize) -> Result<(), Error> {
        let expected = self.arity();
        if expected == actual {
            Ok(())
        } else {
            Err(format!(
                "{} has {} arguments, expected {}",
                self, actual, expected,
            ).into())
        }
    }
}

impl fmt::Display for SpecialForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            SpecialForm::Quote => "quote",
            SpecialForm::If => "if",
            SpecialForm::Defun => "defun",
            SpecialForm::Dethm => "dethm",
        };
        s.fmt(f)
    }
}

/// An abstract syntax tree representing the subset of Scheme used by J. Bob.
/// This is kept deliberately simplified, probably to make it easier to write
/// proofs.
#[cfg_attr(test, derive(Debug))]
pub enum Ast {
    /// A literal value.
    Value(Value),
    /// A variable.
    Variable(Symbol),
    /// Call a function with the specified arguments.
    Apply { func: Box<Ast>, arguments: Vec<Ast> },
    /// Conditionally execute code.
    If { question: Box<Ast>, answer: Box<Ast>, else_: Box<Ast> },
    /// Define a named function with named parameters and a body.
    Define { name: Symbol, parameters: Rc<Vec<Symbol>>, body: Rc<Ast> },
}

impl Ast {
    /// Build an AST from a value.
    pub fn build(ctx: &mut Context, value: &Value) -> Result<Ast, Error> {
        match value {
            Value::Integer(i) => Ok(Ast::Value(Value::Integer(*i))),
            Value::Symbol(s) => Ok(Ast::Variable(s.to_owned())),
            Value::Null => Err("the expression () needs to be quoted".into()),
            Value::Cons(l) => {
                let car = &l.0;
                if let Some(special_form) = SpecialForm::from_value(car) {
                    let cdr = l.1.iter().collect::<Result<Vec<Value>, Error>>()?;
                    special_form.check_arity(cdr.len())?;
                    match special_form {
                        SpecialForm::Quote => {
                            Ok(Ast::Value(cdr[0].clone()))
                        }
                        SpecialForm::If => {
                            let question = Box::new(Ast::build(ctx, &cdr[0])?);
                            let answer = Box::new(Ast::build(ctx, &cdr[1])?);
                            let else_ = Box::new(Ast::build(ctx, &cdr[2])?);
                            Ok(Ast::If { question, answer, else_ })
                        }
                        SpecialForm::Defun | SpecialForm::Dethm => {
                            let parameters = cdr[1].iter()
                                .map(|p| {
                                    let p = p?;
                                    p.expect_symbol()
                                })
                                .collect::<Result<Vec<_>, _>>()?;
                            Ok(Ast::Define {
                                name: cdr[0].expect_symbol()?,
                                parameters: Rc::new(parameters),
                                body: Rc::new(Ast::build(ctx, &cdr[2])?)
                            })
                        }
                    }
                } else {
                    let func = Box::new(Ast::build(ctx, car)?);
                    let arguments = l.1.iter()
                        .map(|arg| {
                            let arg = arg?;
                            Ast::build(ctx, &arg)
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(Ast::Apply { func, arguments })
                }
            }
            // We shouldn't normally see functions here, but if we do, just
            // treat them as literal values.
            f @ Value::Function(_) => Ok(Ast::Value(f.to_owned())),
        }
    }
}
