use std::iter::Peekable;

use crate::ast::{AbstractSyntaxNode, Enum, Identifier};
use crate::lexer::{LexToken, Lexer};

#[derive(Debug)]
pub struct Parser {
    /// The string APPEARS leaked, but this is actually
    /// just to cheat and simplify the borrow checker dramatically.
    /// However, as a result, we **must** ensure that on drop, the box is recreated and the string is dropped.
    input: &'static str,

    /// We use this lexer to lex stuff
    lexer: Peekable<Lexer>,
}

impl Parser {
    /// Leaks the string and creates a new GmlParser
    pub fn new(input: String) -> Self {
        let input = Box::leak(input.into_boxed_str());
        Self {
            input,
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn parse(&mut self) -> Option<AbstractSyntaxNode> {
        if let Some(token) = self.next_lex() {
            let output = match token {
                LexToken::Enum => {
                    let enum_name = self
                        .parse()
                        .expect("expected IDENTIFIER")
                        .unwrap_identifier();

                    assert_eq!(
                        self.next_lex().expect("expected LEFT_BRACE"),
                        LexToken::LeftBrace
                    );

                    let mut variants = Vec::new();
                    let mut comma_latch = false;

                    loop {
                        match self.next_lex() {
                            Some(LexToken::Ident(i)) => {
                                if comma_latch {
                                    panic!("expected RIGHT_BRACE or COMMA, found IDENTIFIER");
                                }
                                comma_latch = true;
                                variants.push(Identifier::new(i))
                            }
                            Some(LexToken::RightBrace) => break,
                            Some(LexToken::Comma) => {
                                // this only happens if we hit a COMMA and then comma AGAIN
                                if comma_latch == false {
                                    panic!("expected IDENTIFIER or RIGHT_BRACE, found COMMA");
                                }
                                // undo the latch...we can have another variant!
                                comma_latch = false;
                            }
                            Some(LexToken::Comment(_)) | Some(LexToken::Whitespace(_)) => {
                                unimplemented!()
                            }
                            None | Some(LexToken::Enum) | Some(LexToken::LeftBrace) => {
                                panic!("expected IDENTIFIER or RIGHT_BRACE");
                            }
                        }
                    }

                    AbstractSyntaxNode::Enum(Enum::new(enum_name, variants))
                }

                LexToken::Comment(_) | LexToken::Whitespace(_) => unimplemented!(),
                LexToken::Ident(ident) => AbstractSyntaxNode::Identifier(Identifier::new(ident)),
                o => panic!("Unexpected Token: {:?}. We expected ENUM", o),
            };

            return Some(output);
        }

        None
    }

    fn next_lex(&mut self) -> Option<LexToken> {
        while let Some(tok) = self.lexer.next() {
            match tok {
                LexToken::Comment(_) | LexToken::Whitespace(_) => {}
                other => return Some(other),
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Lexes the given input string.
    fn parse(input: &'static str) -> Vec<AbstractSyntaxNode> {
        let mut parser = Parser::new(input.to_string());
        let mut output = Vec::new();

        while let Some(v) = parser.parse() {
            output.push(v);
        }

        output
    }

    #[test]
    fn parse_simple() {
        // raw
        assert_eq!(
            parse("enum FooBar {One,Two}"),
            &[AbstractSyntaxNode::Enum(Enum::new(
                Identifier::new("FooBar"),
                vec![Identifier::new("One"), Identifier::new("Two"),]
            ))]
        );

        // whitespace...
        assert_eq!(
            parse(
                "enum         FooBar\n   {One,

                Two

            }"
            ),
            &[AbstractSyntaxNode::Enum(Enum::new(
                Identifier::new("FooBar"),
                vec![Identifier::new("One"), Identifier::new("Two"),]
            ))]
        );

        // comma...
        assert_eq!(
            parse(
                "enum         FooBar\n   {One,

                        Two
                    ,
                    }"
            ),
            &[AbstractSyntaxNode::Enum(Enum::new(
                Identifier::new("FooBar"),
                vec![Identifier::new("One"), Identifier::new("Two"),]
            ))]
        );
    }

    #[test]
    fn parse_comments() {
        // raw
        assert_eq!(
            parse(
                "enum
            // this is obnoxious
            FooBar {One,
                /* good comment */
                Two}"
            ),
            &[AbstractSyntaxNode::Enum(Enum::new(
                Identifier::new("FooBar"),
                vec![Identifier::new("One"), Identifier::new("Two"),]
            ))]
        );
    }

    #[test]
    fn empty_enum() {
        // raw
        assert_eq!(
            parse("enum FooBar {}"),
            &[AbstractSyntaxNode::Enum(Enum::new(
                Identifier::new("FooBar"),
                vec![]
            ))]
        );
    }

    #[test]
    fn whole_thing() {
        let input = "
/// This is an enum to help identify sounds in
/// game.
enum SoundId {
    // haha good laugh!
    JuniperLaughing,

    /// Pretty good sound
    GabeSound,
}
";

        assert_eq!(
            parse(input),
            &[AbstractSyntaxNode::Enum(Enum::new(
                Identifier::new("SoundId"),
                vec![
                    Identifier::new("JuniperLaughing"),
                    Identifier::new("GabeSound"),
                ]
            ))]
        );
    }
}
