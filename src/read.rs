//! A primitive Scheme "reader" function.

use grammar;
use types::Value;

/// Given a string containing a Scheme data structure, parse it
/// and return a Scheme value.
pub(crate) fn read_str(input: &str) -> grammar::ParseResult<Value> {
    grammar::sexpr(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_integers() {
        assert_eq!(read_str("1").unwrap(), Value::Integer(1));
        assert_eq!(read_str("+2").unwrap(), Value::Integer(2));
        assert_eq!(read_str(" -20 ").unwrap(), Value::Integer(-20));
    }

    #[test]
    fn parse_symbols() {
        assert_eq!(read_str("a").unwrap(), Value::symbol("a"));
        assert_eq!(read_str("a-b").unwrap(), Value::symbol("a-b"));
    }

    #[test]
    fn parse_lists() {
        assert_eq!(read_str("()").unwrap(), Value::Null);
        assert_eq!(read_str(" ( ) ").unwrap(), Value::Null);
        assert_eq!(
            read_str("(1)").unwrap(),
            Value::cons(Value::Integer(1), Value::Null),
        );
        assert_eq!(
            read_str("(1 2)").unwrap(),
            Value::cons(
                Value::Integer(1),
                Value::cons(
                    Value::Integer(2),
                    Value::Null,
                ),
            ),
        );
        assert_eq!(
            read_str("(1 . 2)").unwrap(),
            Value::cons(
                Value::Integer(1),
                Value::Integer(2),
            ),
        );

        assert_eq!(
            read_str("(a-b)").unwrap(),
            Value::cons(Value::symbol("a-b"), Value::Null),
        );

        assert!(read_str("(. 2)").is_err());
    }

    #[test]
    fn parse_quote() {
        assert_eq!(read_str("'a").unwrap(), Value::quote(Value::symbol("a")));
    }
}
