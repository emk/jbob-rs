//! Error-handling code.

use std::{error, fmt};
use wasm_bindgen::JsValue;

/// A Scheme runtime error. For simplicity, we implement this as a string.
#[derive(Debug)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for Error {}

impl<'a> From<String> for Error {
    fn from(msg: String) -> Self {
        Error(msg)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(msg: &'a str) -> Self {
        Error(msg.to_owned())
    }
}

impl From<Error> for JsValue {
    fn from(err: Error) -> JsValue {
        err.0.into()
    }
}
