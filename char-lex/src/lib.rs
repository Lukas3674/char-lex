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
//! `Char-Lex` is a crate for easely creating a `char` based lexer from multiple custom enums!
//!
//! ## Example
//!
//! ```rust
//! use char_lex::prelude::*;
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
//!     let lexer: Lexer<Digit, Digit> = Lexer::new("189");
//!     let tokens: Vec<Digit> = lexer.collect();
//!     
//!     assert_eq!(vec![Digit::One, Digit::Eight, Digit::Nine], tokens);
//! }
//! ```
//!
//! `Tokens` can also be wrapped in anything that implements the `TokenWrapper<T>` trait!
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
//!     let lexer: Lexer<Token, Wrapper> = Lexer::new("1");
//!     let tokens: Vec<Wrapper> = lexer.collect();
//!     
//!     assert_eq!(vec![Wrapper { token: Token::One, character: '1' }], tokens);
//! }
//! ```

/// Prelude module.
/// It renames `Error` to `LexErr`!
pub mod prelude {
    pub use crate::{error::Error as LexErr, utils::*, *};
}

/// Contains the `Error` type.
pub mod error;

/// Contains utility types!
pub mod utils;

pub use char_lex_macro::token;
pub use traits::{TokenMatch, TokenTrait, TokenWrapper};

mod traits;

use error::Error;
use std::marker::PhantomData;
use utils::Context;

/// The main lexer type.
///
/// # Generics
/// `T`: `TokenTrait` is the trait implemented by `token` attribute macro.
/// `W`: `TokenWrapper<T>` is the trait that can wrap any token to contain more information,
/// all `TokenTrait` objects automatically implement `TokenWrapper<Self>`, so you don't necessarily need a wrapper!
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Lexer<'l, T, W>
where
    T: TokenTrait,
    W: TokenWrapper<T>,
{
    cursor: usize,
    content: &'l str,
    pos: (usize, usize),
    error: Option<Error>,
    pd: PhantomData<(T, W)>,
}

impl<'l, T, W> Lexer<'l, T, W>
where
    T: TokenTrait,
    W: TokenWrapper<T>,
{
    /// Create a new `Lexer<'l, T, W>` with th `content: &str` that is to be tokenized.
    pub fn new(content: &'l str) -> Self {
        Self {
            content,
            cursor: 0,
            pos: (1, 0),
            error: None,
            pd: PhantomData,
        }
    }

    /// Like the `next` method but with the possibility to ignore certain `Tokens`
    /// by giving a `TokenMatch<T>` like a single `Token` or multiple `vec![Tokens]`.
    pub fn next_ignored<M>(&mut self, m: M) -> Option<W>
    where
        M: TokenMatch<T>,
    {
        loop {
            let (t, c) = self.next_token()?;
            if !m.matches_token(&t) {
                break Some(<W as TokenWrapper<T>>::wrap(t, Context::new(c, self.pos)));
            }
        }
    }

    /// Returns the `Error` that was the reason for the lexer to return `None` from any `next` method!
    pub fn get_error(&self) -> Option<&Error> {
        self.error.as_ref()
    }

    /// Returns the current cursor position.
    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    /// Sets the new cursor position.
    pub fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor
    }

    fn next_token(&mut self) -> Option<(T, char)> {
        if let None = self.error {
            self.cursor += 1;
            if let Some(c) = next_char(self.content, self.cursor) {
                self.pos.1 += 1;
                if c == '\n' {
                    self.pos.0 += 1;
                    self.pos.1 = 0;
                }
                if let Some(t) = <T as TokenTrait>::match_char(c) {
                    Some((t, c))
                } else {
                    self.error = Some(Error::Unexpected(Context::new(c, self.pos)));
                    None
                }
            } else {
                self.error = Some(Error::EndOfFile);
                None
            }
        } else {
            None
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
        let (t, c) = self.next_token()?;
        Some(<W as TokenWrapper<T>>::wrap(t, Context::new(c, self.pos)))
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
