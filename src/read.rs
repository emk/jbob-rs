//! A primitive Scheme "reader" function.

use grammar;
use types::{Context, Value};

/// Given a string containing a Scheme data structure, parse it
/// and return a Scheme value.
pub(crate) fn read_str(
    ctx: &mut Context,
    input: &str,
) -> grammar::ParseResult<Value> {
    grammar::sexpr(input, ctx)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_integers() {
        let mut ctx = Context::default();
        assert_eq!(read_str(&mut ctx, "1").unwrap(), Value::Integer(1));
        assert_eq!(read_str(&mut ctx, "+2").unwrap(), Value::Integer(2));
        assert_eq!(read_str(&mut ctx, " -20 ").unwrap(), Value::Integer(-20));
    }

    #[test]
    fn parse_symbols() {
        let mut ctx = Context::default();
        assert_eq!(read_str(&mut ctx, "a").unwrap(), ctx.intern("a"));
        assert_eq!(read_str(&mut ctx, "a-b").unwrap(), ctx.intern("a-b"));
    }

    #[test]
    fn parse_lists() {
        let mut ctx = Context::default();
        assert_eq!(read_str(&mut ctx, "()").unwrap(), Value::Null);
        assert_eq!(read_str(&mut ctx, " ( ) ").unwrap(), Value::Null);
        assert_eq!(
            read_str(&mut ctx, "(1)").unwrap(),
            Value::cons(Value::Integer(1), Value::Null),
        );
        assert_eq!(
            read_str(&mut ctx, "(1 2)").unwrap(),
            Value::cons(
                Value::Integer(1),
                Value::cons(
                    Value::Integer(2),
                    Value::Null,
                ),
            ),
        );
        assert_eq!(
            read_str(&mut ctx, "(1 . 2)").unwrap(),
            Value::cons(
                Value::Integer(1),
                Value::Integer(2),
            ),
        );

        assert_eq!(
            read_str(&mut ctx, "(a-b)").unwrap(),
            Value::cons(ctx.intern("a-b"), Value::Null),
        );

        assert!(read_str(&mut ctx, "(. 2)").is_err());
    }

    #[test]
    fn parse_quote() {
        let mut ctx = Context::default();
        let a = ctx.intern("a");
        assert_eq!(
            read_str(&mut ctx, "'a").unwrap(),
            Value::quote(&mut ctx, a),
        );
    }
}
