//! Callable Scheme functions.

use std::{cell::RefCell, fmt, rc::Rc};

use context::Context;
use environment::Environment;
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

    /// Define a function in the specified `Context`.
    pub fn define<F>(
        env: &Rc<RefCell<Environment>>,
        name: Symbol,
        arity: usize,
        func: F,
    )
    where
        F: Fn(&mut Context, &[Value]) -> Result<Value, Error> + 'static,
    {
        let value = Value::Function(Rc::new(
            Function::new(Some(name.clone()), arity, func)
        ));
        env.borrow_mut().define(name, value);
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

/// Add the necessary Scheme functions to the specified context.
pub fn add_prelude_functions(ctx: &mut Context) {
    let env = ctx.global_environment();

    Function::define(&env, ctx.intern_symbol("atom"), 1, |ctx, args| {
        if let Value::Cons(_) = args[0] {
            Ok(ctx.nil())
        } else {
            Ok(ctx.t())
        }
    });

    Function::define(&env, ctx.intern_symbol("cons"), 2, |_ctx, args| {
        Ok(Value::Cons(Rc::new((args[0].to_owned(), args[1].to_owned()))))
    });

    Function::define(&env, ctx.intern_symbol("car"), 1, |_ctx, args| {
        if let Value::Cons(c) = &args[0] {
            Ok(c.0.clone())
        } else {
            Ok(Value::Null)
        }
    });

    Function::define(&env, ctx.intern_symbol("cdr"), 1, |_ctx, args| {
        if let Value::Cons(c) = &args[0] {
            Ok(c.1.clone())
        } else {
            Ok(Value::Null)
        }
    });

    Function::define(&env, ctx.intern_symbol("equal"), 2, |ctx, args| {
        Ok(ctx.bool_value(args[0] == args[1]))
    });

    Function::define(&env, ctx.intern_symbol("natp"), 1, |ctx, args| {
        if let Value::Integer(i) = &args[0] {
            Ok(ctx.bool_value(*i >= 0))
        } else {
            Ok(ctx.nil())
        }
    });

    Function::define(&env, ctx.intern_symbol("+"), 2, |_ctx, args| {
        Ok(Value::Integer(num(&args[0]) + num(&args[1])))
    });

    Function::define(&env, ctx.intern_symbol("<"), 2, |ctx, args| {
        Ok(ctx.bool_value(num(&args[0]) < num(&args[1])))
    });

    ctx.eval_file("
    (defun size (x)
      (if (atom x)
        '0
        (+ '1 (+ (size (car x)) (size (cdr x))))))
    ").unwrap();
}

/// If `value` is an integer, return it. Otherwise, return 0. This is for
/// compatibility with how J-Bob defines total functions.
fn num(value: &Value) -> i64 {
    match value {
        Value::Integer(i) => *i,
        _ => 0,
    }
}
