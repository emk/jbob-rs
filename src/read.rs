//! A primitive Scheme "reader" function.

use grammar;
use types::{Context, Error, Value};

/// Given a string containing a Scheme data structure, parse it
/// and return a Scheme value.
pub fn read_sexpr(
    ctx: &mut Context,
    input: &str,
) -> Result<Value, Error> {
    grammar::sexpr(input, ctx).map_err(|err| pretty_parse_error(input, err))
}

/// Given a string containing multiple top-level Scheme definitions, read
/// them as a vector of Scheme data structures.
pub fn read_file(
    ctx: &mut Context,
    input: &str,
) -> Result<Vec<Value>, Error> {
    grammar::file(input, ctx).map_err(|err| pretty_parse_error(input, err))
}

/// Given a `ParseError`, turn in into a pretty, multi-line `Error`.
fn pretty_parse_error(input: &str, err: grammar::ParseError) -> Error {
    // Convert 1-based line and column to 0-based numbers.
    let lineno = err.line.checked_sub(1).expect("no such line 0");
    let colno = err.column.checked_sub(1).expect("no such column 0");

    // Get the actual source line.
    let line = input.lines().nth(lineno).expect("line is not in input");

    // Build a pretty error message.
    let location = format!("{}:{}: ", err.line, err.column);
    let msg = format!(
        "parse error, expected one of {:?}\n{}{}\n{}^",
        err.expected,
        location,
        line.trim_right(),
        " ".repeat(location.len() + colno),
    );
    Error(msg)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_integers() {
        let mut ctx = Context::default();
        assert_eq!(read_sexpr(&mut ctx, "1").unwrap(), Value::Integer(1));
        assert_eq!(read_sexpr(&mut ctx, "+2").unwrap(), Value::Integer(2));
        assert_eq!(read_sexpr(&mut ctx, " -20 ").unwrap(), Value::Integer(-20));
    }

    #[test]
    fn parse_symbols() {
        let mut ctx = Context::default();
        assert_eq!(read_sexpr(&mut ctx, "a").unwrap(), ctx.intern("a"));
        assert_eq!(read_sexpr(&mut ctx, "a-b").unwrap(), ctx.intern("a-b"));
    }

    #[test]
    fn parse_lists() {
        let mut ctx = Context::default();
        assert_eq!(read_sexpr(&mut ctx, "()").unwrap(), Value::Null);
        assert_eq!(read_sexpr(&mut ctx, " ( ) ").unwrap(), Value::Null);
        assert_eq!(
            read_sexpr(&mut ctx, "(1)").unwrap(),
            Value::cons(Value::Integer(1), Value::Null),
        );
        assert_eq!(
            read_sexpr(&mut ctx, "(1 2)").unwrap(),
            Value::cons(
                Value::Integer(1),
                Value::cons(
                    Value::Integer(2),
                    Value::Null,
                ),
            ),
        );
        assert_eq!(
            read_sexpr(&mut ctx, "(1 . 2)").unwrap(),
            Value::cons(
                Value::Integer(1),
                Value::Integer(2),
            ),
        );

        assert_eq!(
            read_sexpr(&mut ctx, "(a-b)").unwrap(),
            Value::cons(ctx.intern("a-b"), Value::Null),
        );

        assert!(read_sexpr(&mut ctx, "(. 2)").is_err());
    }

    #[test]
    fn parse_quote() {
        let mut ctx = Context::default();
        let a = ctx.intern("a");
        assert_eq!(
            read_sexpr(&mut ctx, "'a").unwrap(),
            Value::quote(&mut ctx, a),
        );
    }

    #[test]
    fn parse_file() {
        let mut ctx = Context::default();
        assert_eq!(
            read_file(&mut ctx, "1 2").unwrap(),
            vec![Value::Integer(1), Value::Integer(2)],
        )
    }

    #[test]
    fn parse_jbob_source() {
        let mut ctx = Context::default();
        read_file(&mut ctx, include_str!("scheme/j-bob.scm")).unwrap();
    }

    //#[test]
    //fn parse_little_prover_source() {
    //    let mut ctx = Context::default();
    //    read_file(&mut ctx, include_str!("scheme/little-prover.scm")).unwrap();
    //}
}
