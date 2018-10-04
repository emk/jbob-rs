//! Evaluate a Scheme expression in a context.

use std::{
    cell::RefCell,
    rc::Rc,
};

use ast::Ast;
use context::Context;
use environment::Environment;
use errors::Error;
use functions::Function;
use types::Value;

/// Evaluate the AST in `ctx`, updating it appropriately, and returning the
/// resulting value.
pub fn eval_ast(
    ctx: &mut Context,
    env: &Rc<RefCell<Environment>>,
    ast: &Ast,
) -> Result<Value, Error> {
    match ast {
        Ast::Value(value) => Ok(value.to_owned()),
        Ast::Variable(symbol) => {
            env.borrow().lookup(symbol).ok_or_else(|| {
                format!("variable `{}` is not defined", symbol).into()
            })
        }
        Ast::Apply { func, arguments } => {
            let func = eval_ast(ctx, env, func)?;
            let arguments = arguments.iter()
                .map(|ast| eval_ast(ctx, env, ast))
                .collect::<Result<Vec<_>, _>>()?;
            if let Value::Function(func) = func {
                func.call(ctx, &arguments)
            } else {
                Err(format!("cannot call {} as function", func).into())
            }
        }
        Ast::If { question, answer, else_ } => {
            if eval_ast(ctx, env, question)? != ctx.nil() {
                eval_ast(ctx, env, answer)
            } else {
                eval_ast(ctx, env, else_)
            }
        }
        Ast::Define { name, parameters, body } => {
            let body = {
                let env = env.to_owned();
                let name = name.to_owned();
                let parameters = parameters.to_owned();
                let body = body.to_owned();
                move |ctx: &mut Context, arguments: &[Value]| {
                    let env = Environment::make_child(env.clone());
                    for (param, arg) in parameters.iter().zip(arguments) {
                        env.borrow_mut()
                            .define(param.to_owned(), arg.to_owned());
                    }
                    eval_ast(ctx, &env, &body).map_err(|err| {
                        format!("error in '{}': {}", name, err).into()
                    })
                }
            };
            Function::define(env, name.to_owned(), parameters.len(), body);
            Ok(Value::Null)
        }
    }
}
