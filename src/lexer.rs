use std::{iter::Peekable, str::CharIndices};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexToken {
    Enum,
    Ident(&'static str),
    Comment(Comment),
    Whitespace(Whitespace),
    LeftBrace,
    RightBrace,
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
                            }
                        }

                        LexToken::Comment(Comment::Block)
                    } else {
                        panic!(
                            "Unexpected '/', not a start of COMMENT, DOC_COMMENT, or BLOCK COMMENT"
                        );
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
            }

            return true;
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
            lex("hello "),
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
}
