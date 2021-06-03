#![allow(clippy::bool_comparison)]
#![deny(rust_2018_idioms)]

mod ast;
mod lexer;
mod parser;

pub use ast::*;
pub use lexer::*;
pub use parser::*;
