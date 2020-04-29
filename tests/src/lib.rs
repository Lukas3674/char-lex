#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

//! Tests for the `Char-Lex` crate.

use char_lex::prelude::*;

#[token]
#[derive(Debug, PartialEq)]
enum Token {
    Whitespace = [' ', '\t', '\r', '\n'],

    Digit(Digit),
}

#[token]
#[derive(Debug, PartialEq)]
enum Digit {
    Zero = '0',
    One = '1',
    Two = '2',
    Three = '3',
    Four = '4',
    Five = '5',
    Six = '6',
    Seven = '7',
    Eight = '8',
    Nine = '9',
}

#[test]
fn no_wrapper() {
    let lexer: Lexer<Token, Token> = Lexer::new("189");
    let tokens: Vec<Token> = lexer.collect();

    assert_eq!(
        vec![
            Token::Digit(Digit::One),
            Token::Digit(Digit::Eight),
            Token::Digit(Digit::Nine)
        ],
        tokens
    );
}

#[derive(Debug, PartialEq)]
struct Wrapper {
    token: Token,
    character: char,
}

impl TokenWrapper<Token> for Wrapper {
    fn wrap(token: Token, context: Context) -> Self {
        Self {
            token,
            character: context.character,
        }
    }
}

#[test]
fn wrapper() {
    let mut lexer: Lexer<Token, Wrapper> = Lexer::new("1");

    let mut tokens: Vec<Wrapper> = Vec::new();
    for token in &mut lexer {
        tokens.push(token);
    }

    assert_eq!(
        vec![Wrapper {
            token: Token::Digit(Digit::One),
            character: '1'
        }],
        tokens
    );
    assert_eq!(&LexErr::EndOfFile, lexer.get_error().unwrap());
}
