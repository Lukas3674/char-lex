use super::utils::Context;

/// [`Char-Lex`] crate's error type.
///
/// [`Char-Lex`]: ../
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Type that is returned when the content of the [`Lexer<'l, T, W>`] is emtpy.
    ///
    /// [`Lexer<'l, T, W>`]: ../struct.Lexer.html
    EndOfFile,

    /// Type that is returned when the [`Lexer<'l, T, W>`] encounters a [`char`] that is not matched by any `Token` with the [`TokenTrait`].
    ///
    /// [`Lexer<'l, T, W>`]: ../struct.Lexer.html
    /// [`char`]: https://doc.rust-lang.org/std/primitive.char.html
    /// [`TokenTrait`]: ../trait.TokenTrait.html
    Unexpected(Context),
}
