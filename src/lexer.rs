use std::{iter::Peekable, str::CharIndices};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexToken {
    Enum,
    Ident(&'static str),
    Comment(Comment),
    Whitespace(Whitespace),
    Comma,
    LeftBrace,
    RightBrace,
}

impl LexToken {
    /// Shorthand for space for unit tests
    #[cfg(test)]
    pub fn space() -> Self {
        LexToken::Whitespace(Whitespace::Space)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Whitespace {
    Space,
    Tab,
    Newline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comment {
    Doc,
    Line,
    Block,
}

/// Lexes the given input string.
pub fn lex(input: &'static str) -> Vec<LexToken> {
    let mut lexer = Lexer {
        input,
        iter: input.char_indices().peekable(),
    };

    let mut output = Vec::new();

    while let Some(v) = lexer.next_token() {
        output.push(v);
    }

    output
}

struct Lexer {
    input: &'static str,
    iter: Peekable<CharIndices<'static>>,
}

impl Lexer {
    /// Generates a new LexToken...
    pub fn next_token(&mut self) -> Option<LexToken> {
        while let Some((i, c)) = self.iter.next() {
            let token = match c {
                ' ' => LexToken::Whitespace(Whitespace::Space),
                '\t' => LexToken::Whitespace(Whitespace::Tab),
                '\n' => LexToken::Whitespace(Whitespace::Newline),
                ',' => LexToken::Comma,

                '{' => LexToken::LeftBrace,
                '}' => LexToken::RightBrace,

                // we don't care about \r, so we don't support OSes that only use
                // \r as a newline. which is like, macos in 2002 so i think we're good
                '\r' => continue,

                // Comment
                '/' => {
                    if self.try_char('/') {
                        // okay check for DOC COMMENT...
                        let is_doc_comment = self.try_char('/');

                        // MUNCH MUNC MOTHERFUCKER UNTIL IT'S NOT A NEWLINE!
                        while self.try_advance(|v| v != '\n') {}

                        if is_doc_comment {
                            LexToken::Comment(Comment::Doc)
                        } else {
                            LexToken::Comment(Comment::Line)
                        }
                    } else if self.try_char('*') {
                        // gml does not support nested blocked comments!
                        // which is convenient for lazy asses like MYSELF
                        loop {
                            if self.try_char('*') && self.try_char('/') {
                                break;
                            } else {
                                // advance...
                                if self.iter.next().is_none() {
                                    panic!("Unexpected END_OF_FILE, expected END_OF_BLOCK_COMMENT");
                                }
                            }
                        }

                        LexToken::Comment(Comment::Block)
                    } else {
                        panic!("Unexpected '/', expected COMMENT, DOC_COMMENT, or BLOCK COMMENT");
                    }
                }

                'A'..='Z' | 'a'..='z' => {
                    // MANCH!!!!!!!!!!
                    let start = i;

                    // SWAM
                    while self.try_advance(|v| matches!(v, 'A'..='Z' | 'a'..='z')) {}

                    let end = self
                        .iter
                        .peek()
                        .map(|v| v.0)
                        .unwrap_or_else(|| self.input.len());

                    let word = &self.input[start..end];

                    // is the bird the word?
                    match word {
                        "enum" => LexToken::Enum,
                        other => LexToken::Ident(other),
                    }
                }

                // hmm what's that token!
                unexpected => panic!("Unexpected token: {}", unexpected),
            };

            println!("Found token {:?}", token);

            return Some(token);
        }

        None
    }

    /// Tries to take the next char IF it is of the requested char type. Otherwise,
    /// it does not advance the iterator
    fn try_char(&mut self, condition: char) -> bool {
        self.try_advance(|v| v == condition)
    }

    fn try_advance(&mut self, mut condition: impl FnMut(char) -> bool) -> bool {
        if let Some(v) = self.iter.peek() {
            if condition(v.1) {
                self.iter.next();
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lex_control() {
        assert_eq!(lex("enum"), &[LexToken::Enum]);
    }

    #[test]
    fn lex_identifiers() {
        assert_eq!(
            lex("hello there\n very nice\n\n"),
            &[
                LexToken::Ident("hello"),
                LexToken::Whitespace(Whitespace::Space),
                LexToken::Ident("there"),
                LexToken::Whitespace(Whitespace::Newline),
                LexToken::Whitespace(Whitespace::Space),
                LexToken::Ident("very"),
                LexToken::Whitespace(Whitespace::Space),
                LexToken::Ident("nice"),
                LexToken::Whitespace(Whitespace::Newline),
                LexToken::Whitespace(Whitespace::Newline),
            ]
        );
    }

    #[test]
    fn lex_comments() {
        assert_eq!(
            lex("// hey there how are you\n/* this is \n\n\na block * comment */\n/// and this is a doc comment"),
            &[
                LexToken::Comment(Comment::Line),
                LexToken::Whitespace(Whitespace::Newline),
                LexToken::Comment(Comment::Block),
                LexToken::Whitespace(Whitespace::Newline),
                LexToken::Comment(Comment::Doc),
            ]
        );
    }

    #[test]
    fn comment_eof() {
        assert_eq!(lex("// nicetryguy"), &[LexToken::Comment(Comment::Line),]);
    }

    #[test]
    #[should_panic]
    fn bad_block() {
        lex("/* this block will not end correctly");
    }

    #[test]
    fn the_full_nine_yards() {
        assert_eq!(
            lex("\
enum FooBar {
    One,
    Two,
}"),
            &[
                LexToken::Enum,
                LexToken::space(),
                LexToken::Ident("FooBar"),
                LexToken::space(),
                LexToken::LeftBrace,
                LexToken::Whitespace(Whitespace::Newline),
                // four spaces...
                LexToken::space(),
                LexToken::space(),
                LexToken::space(),
                LexToken::space(),
                // Word
                LexToken::Ident("One"),
                LexToken::Comma,
                LexToken::Whitespace(Whitespace::Newline),
                // four spaces...
                LexToken::space(),
                LexToken::space(),
                LexToken::space(),
                LexToken::space(),
                // word
                LexToken::Ident("Two"),
                LexToken::Comma,
                LexToken::Whitespace(Whitespace::Newline),
                // final and out:
                LexToken::RightBrace,
            ]
        );
    }
}
