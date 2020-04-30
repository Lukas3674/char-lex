use super::utils::Context;

/// `Char-Lex` crate's error type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Type that is returned when the content of the `Lexer<'l, T, W>` is emtpy.
    EndOfFile,

    /// Type that is returned when the `Lexer<'l, T, W>` encounters a `char` that is not matched by any `Token`.
    Unexpected(Context),
}
