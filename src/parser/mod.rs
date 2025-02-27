//! The parser for the jsonpath.
//! The module grammar denotes the structure of the parsing grammar

pub(crate) mod model;
#[allow(clippy::module_inception)]
pub(crate) mod parser;
mod macros;

