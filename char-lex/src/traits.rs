use super::utils::Context;

/// The main trait for `Tokens`,
/// it is the automatically implemented by the [`token`] attribute macro.
///
/// [`token`]: https://doc.rust-lang.org/
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
/// [`TokenTrait`]: https://doc.rust-lang.org/
pub trait TokenWrapper<T>
where
    Self: Sized,
    T: TokenTrait,
{
    /// Function that wraps the `Token` with the [`TokenTrait`] and the [`Context`] in itself.
    ///
    /// [`TokenTrait`]: https://doc.rust-lang.org/
    /// [`Context`]: https://doc.rust-lang.org/
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
