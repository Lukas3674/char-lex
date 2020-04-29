use super::utils::Context;

/// The main trait for `Tokens`,
/// it is the automatically implemented by the [`token`] attribute macro.
///
/// [`token`]: https://docs.rs/char-lex-macro/0.1.0/char_lex_macro/attr.token.html
pub trait TokenTrait
where
    Self: Sized + PartialEq,
{
    /// Returns the enum element that matches the [`char`].
    ///
    /// [`char`]: https://doc.rust-lang.org/std/primitive.char.html
    fn match_char(c: char) -> Option<Self>;
}

/// Trait for anything that wants to automatically wrap a `Token` with the [`TokenTrait`],
///
/// [`TokenTrait`]: ../trait.TokenTrait.html
pub trait TokenWrapper<T>
where
    Self: Sized,
    T: TokenTrait,
{
    /// Function that wraps the `Token` with the [`TokenTrait`] and the [`Context`] in itself.
    ///
    /// [`TokenTrait`]: ../trait.TokenTrait.html
    /// [`Context`]: ../utils/struct.Context.html
    fn wrap(token: T, context: Context) -> Self;
}

impl<T> TokenWrapper<T> for T
where
    T: TokenTrait,
{
    fn wrap(token: T, _: Context) -> Self {
        token
    }
}

/// Trait for anything that wants to match a single `Token` with the [`TokenTrait`],
///
/// [`TokenTrait`]: ../trait.TokenTrait.html
pub trait TokenMatch<T>
where
    T: TokenTrait,
{
    /// Function that matches a single `Token` with the [`TokenTrait`].
    ///
    /// [`TokenTrait`]: ../trait.TokenTrait.html
    fn matches_token(&self, t: &T) -> bool;
}

impl<T> TokenMatch<T> for T
where
    T: TokenTrait,
{
    fn matches_token(&self, t: &T) -> bool {
        self == t
    }
}

impl<T> TokenMatch<T> for [T]
where
    T: TokenTrait,
{
    fn matches_token(&self, t: &T) -> bool {
        for token in self {
            if token == t {
                return true;
            }
        }
        false
    }
}
