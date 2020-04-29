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
//! [`Char-Lex`] is a crate for easely creating a [`char`] based lexer from multiple custom enums!
//!
//! ## Example
//!
//! ```rust
//! use char_lex::prelude::*;
//!
//! #[token]
//! #[derive(Debug, PartialEq)]
//! enum Token {
//!     Whitespace = [' ', '\t', '\r', '\n'],
//!
//!     Digit(Digit),
//! }
//!
//! #[token]
//! #[derive(Debug, PartialEq)]
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
//!     let lexer: Lexer<Token, Token> = Lexer::new("1 \r\n 8 9", Some(Token::Whitespace));
//!     let tokens: Vec<Token> = lexer.collect();
//!     
//!     assert_eq!(vec![Token::Digit(Digit::One), Token::Digit(Digit::Eight), Token::Digit(Digit::Nine)], tokens);
//! }
//! ```
//!
//! `Tokens` with the [`TokenTrait`] can also be wrapped in anything that implements the [`TokenWrapper<T>`] trait!
//!
//! ## Example
//!
//! ```rust
//! use char_lex::prelude::*;
//!
//! #[token]
//! #[derive(Debug, PartialEq)]
//! enum Token {
//!     One = '1',
//! }
//!
//! #[derive(Debug, PartialEq)]
//! struct Wrapper {
//!     token: Token,
//!     character: char,
//! }
//!
//! impl TokenWrapper<Token> for Wrapper {
//!     fn wrap(token: Token, context: Context) -> Self {
//!         Self { token, character: context.character }
//!     }
//! }
//!
//! fn main() {
//!     let lexer: Lexer<Token, Wrapper> = Lexer::new("1", None);
//!     let tokens: Vec<Wrapper> = lexer.collect();
//!     
//!     assert_eq!(vec![Wrapper { token: Token::One, character: '1' }], tokens);
//! }
//! ```
//!
//! [`Char-Lex`]: ./
//! [`char`]: https://doc.rust-lang.org/std/primitive.char.html
//! [`TokenWrapper<T>`]: ./trait.TokenWrapper.html

/// Prelude module for [`Char-Lex`].
/// It renames [`Error`] to `LexErr`!
///
/// [`Char-Lex`]: ../
/// [`Error`]: ../error/enum.Error.html
pub mod prelude {
    pub use crate::{error::Error as LexErr, utils::*, *};
}

/// Contains the [`Error`] type for module [`Char-Lex`].
///
/// [`Error`]: ./enum.Error.html
/// [`Char-Lex`]: ../
pub mod error;

/// Contains utility types like [`Context`]!
///
/// [`Context`]: ./struct.Context.html
pub mod utils;

pub use char_lex_macro::token;
pub use traits::{TokenTrait, TokenWrapper};

mod traits;

use error::Error;
use std::marker::PhantomData;
use utils::Context;

/// The main lexer type from the module [`Char-Lex`].
///
/// # Generics
/// `T`: [`TokenTrait`] is the trait implemented by [`token`] attribute macro.
/// `W`: [`TokenWrapper<T>`] is the trait that can wrap a token to contain more information,
/// all [`TokenTrait`] objects automatically implement [`TokenWrapper<T>`], so you don't need a wrapper!
///
/// [`Char-Lex`]: ./
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
    ignored: Option<T>,
    pos: (usize, usize),
    error: Option<Error>,
    pd: PhantomData<(T, W)>,
}

impl<'l, T, W> Lexer<'l, T, W>
where
    T: TokenTrait,
    W: TokenWrapper<T>,
{
    /// Create new [`Lexer<'l, T, W>`]
    ///
    /// # Arguments
    /// `content`: the string that is to be tokenized.
    /// `ignored`: an optional token that the lexer will ignore and give you the next.
    ///
    /// [`Lexer<'l, T, W>`]: ./struct.Lexer.html
    pub fn new(content: &'l str, ignored: Option<T>) -> Self {
        Self {
            content,
            ignored,
            cursor: 0,
            pos: (1, 0),
            error: None,
            pd: PhantomData,
        }
    }

    /// Returns the [`Error`] that was the reason for the lexer to return `None` from the `next()` method!
    ///
    /// [`Error`]: ./error/enum.Error.html
    pub fn get_error(&self) -> Option<&Error> {
        self.error.as_ref()
    }

    /// Returns the current cursor position.
    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    /// Sets the new cursor position.
    /// Returns an [`Error`] if the cursor is out of `content` bounds!
    ///
    /// [`Error`]: ./error/enum.Error.html
    pub fn set_cursor(&mut self, cursor: usize) -> Result<(), Error> {
        if cursor > 0 && cursor < self.content.len() {
            self.cursor = cursor;
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }
}

impl<'l, T, W> Iterator for Lexer<'l, T, W>
where
    T: TokenTrait,
    W: TokenWrapper<T>,
{
    type Item = W;

    fn next(&mut self) -> Option<Self::Item> {
        if let None = self.error {
            loop {
                self.cursor += 1;
                if let Some(c) = next_char(self.content, self.cursor) {
                    self.pos.1 += 1;
                    if c == '\n' {
                        self.pos.0 += 1;
                        self.pos.1 = 0;
                    }
                    if let Some(t) = <T as TokenTrait>::match_char(c) {
                        if let Some(i) = self.ignored.as_ref() {
                            if &t != i {
                                break Some(<W as TokenWrapper<T>>::wrap(
                                    t,
                                    Context::new(c, self.pos),
                                ));
                            }
                        } else {
                            break Some(<W as TokenWrapper<T>>::wrap(t, Context::new(c, self.pos)));
                        }
                    } else {
                        self.cursor -= 1;
                        self.error = Some(Error::Unexpected(Context::new(c, self.pos)));
                        break None;
                    }
                } else {
                    self.error = Some(Error::EndOfFile);
                    break None;
                }
            }
        } else {
            None
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
