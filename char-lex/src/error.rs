use super::utils::Context;

/// Error type for module [`char_lex`].
///
/// [`char_lex`]: https://doc.rust-lang.org/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Type that is returned when the content of the [`Lexer<'l, T, W>`] is emtpy.
    ///
    /// [`Lexer<'l, T, W>`]: https://doc.rust-lang.org/
    EndOfFile,

    /// Type that is returned when the [`Lexer<'l, T, W>`] encounters a [`char`] that is not matched by any `Token` with the [`TokenTrait`].
    ///
    /// [`Lexer<'l, T, W>`]: https://doc.rust-lang.org/
    /// [`char`]: https://doc.rust-lang.org/
    /// [`TokenTrait`]: https://doc.rust-lang.org/
    Unexpected(Context),
}
