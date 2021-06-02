#![allow(clippy::bool_comparison)]
#![deny(rust_2018_idioms)]

mod lexer;
mod parser;

#[derive(Debug)]
pub struct GmlCompiler {
    /// The string APPEARS leaked, but this is actually
    /// just to cheat and simplify the borrow checker dramatically.
    /// However, as a result, we **must** ensure that on drop, the box is recreated and the string is dropped.
    input: &'static str,
}

impl GmlCompiler {
    pub fn new(input: String) -> Self {
        let input = Box::leak(input.into_boxed_str());
        let me = Self { input };

        lexer::lex(me.input);

        me
    }
}
