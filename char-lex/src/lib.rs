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

//! # CHAR-LEX
//!
//! [`char_lex`] is a crate for easely creating a [`char`] based lexer from multiple custom enums!
//!
//! ## Example
//!
//! ```rust
//! use char_lex::prelude::*;
//!
//! #[token]
//! #[derive(PartialEq)]
//! enum Token {
//!     Whitespace = [' ', '\t', '\r', '\n'],
//!
//!     Digit(Digit),
//! }
//!
//! #[token]
//! #[derive(PartialEq)]
//! enum Digit {
//!     Zero = '0',
//!     One = '1',
//!     Two = '2',
//!     Three = '3',
//!     Four = '4',
//!     Five = '5',
//!     Six = '6',
//!     Seven = '7',
//!     Eight = '8',
//!     Nine = '9',
//! }
//!
//! fn main() {
//!     let mut lexer: Lexer<Token, Token> = Lexer::new("1 \r\n 8 9");
//!
//!     let mut tokens = Vec::new();
//!     while let Ok(t) = lexer.poll(Some(Token::Whitespace)) {
//!         tokens.push(t);
//!     }
//!     
//!     assert_eq!(Err(LexErr::EndOfFile), lexer.poll(None));
//!     assert_eq!(vec![Token::Digit(Digit::One), Token::Digit(Digit::Eight), Token::Digit(Digit::Nine)], tokens);
//! }
//! ```
//!
//! `Tokens` with the [`TokenTrait`] can also be wrapped in anything that implements the [`TokenWrapper<T>`] trait!
//!
//! ## Example
//!
//! ```rust
//! #[derive(PartialEq)]
//! struct Wrapper<Token> {
//!     token: Token,
//!     character: char,
//! }
//!
//! impl TokenWrapper<Token> for Wrapper {
//!     fn wrap(token: T, context: Context) -> Self {
//!         Self { token, character: context.character }
//!     }
//! }
//!
//! fn main() {
//!     let mut lexer: Lexer<Token, Wrapper> = Lexer::new("1");
//!
//!     let mut tokens = Vec::new();
//!     while let Ok(t) = lexer.poll(Some(Token::Whitespace)) {
//!         tokens.push(t);
//!     }
//!     
//!     assert_eq!(Err(LexErr::EndOfFile), lexer.poll(None));
//!     assert_eq!(vec![Wrapper { token: Token::Digit(Digit::One), character: '1' }], tokens);
//! }
//! ```
//!
//! [`char_lex`]: ./
//! [`char`]: https://doc.rust-lang.org/std/primitive.char.html
//! [`TokenWrapper<T>`]: ./trait.TokenWrapper.html

/// Prelude module for [`char_lex`].
/// It renames [`Error`] to `LexErr`!
///
/// [`char_lex`]: ./
/// [`Error`]: ./error/enum.Error.html
pub mod prelude {
    pub use crate::{error::Error as LexErr, utils::*, *};
}

/// Contains the [`Error`] type for module [`char_lex`].
///
/// [`Error`]: ./error/enum.Error.html
/// [`char_lex`]: ./
pub mod error;

/// Contains utility types like [`Context`]!
///
/// [`Context`]: ./utils/struct.Context.html
pub mod utils;

pub use char_lex_macro::token;
pub use traits::{TokenTrait, TokenWrapper};

mod traits;

use error::Error;
use std::marker::PhantomData;
use utils::Context;

/// The main lexer type from the module [`char_lex`].
///
/// # Generics
/// `T`: [`TokenTrait`] is the trait implemented by [`token`] attribute macro.
/// `W`: [`TokenWrapper<T>`] is the trait that can wrap a token to contain more information,
/// all [`TokenTrait`] objects automatically implement [`TokenWrapper<T>`], so you don't need a wrapper!
///
/// [`char_lex`]: ./
/// [`TokenTrait`]: ./trait.TokenTrait.html
/// [`token`]: https://docs.rs/char-lex-macro/0.1.0/char_lex_macro/attr.token.html
/// [`TokenWrapper<T>`]: ./trait.TokenWrapper.html
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Lexer<'l, T, W>
where
    T: TokenTrait,
    W: TokenWrapper<T>,
{
    cursor: usize,
    content: &'l str,
    pos: (usize, usize),
    pd: PhantomData<(T, W)>,
}

impl<'l, T, W> Lexer<'l, T, W>
where
    T: TokenTrait,
    W: TokenWrapper<T>,
{
    /// Create new [`Lexer<'l, T, W>`]
    ///
    /// [`Lexer<'l, T, W>`]: ./struct.Lexer.html
    pub fn new(content: &'l str) -> Self {
        Self {
            content,
            cursor: 0,
            pos: (1, 0),
            pd: PhantomData,
        }
    }

    /// Only peek at the next token / wrapper (`W`: [`TokenWrapper<T>`]),
    /// the next peek will be the same as the previous!
    ///
    /// Only a `&self` reference is required here!
    ///
    /// [`TokenWrapper<T>`]: ./trait.TokenWrapper.html
    pub fn peek(&self, ignored: Option<T>) -> Result<W, Error> {
        let mut pos = self.pos;
        let mut cursor = self.cursor;
        loop {
            cursor += 1;
            if let Some(c) = next_char(self.content, cursor) {
                pos.1 += 1;
                if c == '\n' {
                    pos.0 += 1;
                    pos.1 = 0;
                }
                if let Some(t) = <T as TokenTrait>::match_char(c) {
                    if let Some(i) = &ignored {
                        if &t != i {
                            break Ok(<W as TokenWrapper<T>>::wrap(t, Context::new(c, pos)));
                        }
                    } else {
                        break Ok(<W as TokenWrapper<T>>::wrap(t, Context::new(c, pos)));
                    }
                } else {
                    break Err(Error::Unexpected(Context::new(c, self.pos)));
                }
            } else {
                break Err(Error::EndOfFile);
            }
        }
    }

    /// Poll the next token / wrapper (`W`: [`TokenWrapper<T>`]),
    ///
    /// A `&mut self` reference is required here!
    ///
    /// [`TokenWrapper<T>`]: ./trait.TokenWrapper.html
    pub fn poll(&mut self, ignored: Option<T>) -> Result<W, Error> {
        loop {
            self.cursor += 1;
            if let Some(c) = next_char(self.content, self.cursor) {
                self.pos.1 += 1;
                if c == '\n' {
                    self.pos.0 += 1;
                    self.pos.1 = 0;
                }
                if let Some(t) = <T as TokenTrait>::match_char(c) {
                    if let Some(i) = &ignored {
                        if &t != i {
                            break Ok(<W as TokenWrapper<T>>::wrap(t, Context::new(c, self.pos)));
                        }
                    } else {
                        break Ok(<W as TokenWrapper<T>>::wrap(t, Context::new(c, self.pos)));
                    }
                } else {
                    self.cursor -= 1;
                    break Err(Error::Unexpected(Context::new(c, self.pos)));
                }
            } else {
                break Err(Error::EndOfFile);
            }
        }
    }
}

fn next_char<'l>(content: &'l str, cursor: usize) -> Option<char> {
    if cursor <= content.len() {
        let (c, _) = content.split_at(cursor);
        Some(c.chars().last().unwrap())
    } else {
        None
    }
}
