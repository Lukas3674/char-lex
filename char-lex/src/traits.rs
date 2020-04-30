use super::utils::Context;

/// The main trait for `Tokens`,
/// it is the automatically implemented by the `token` attribute macro.
pub trait TokenTrait
where
    Self: Sized + PartialEq,
{
    /// Returns the enum element that matches the given `char`.
    fn match_char(c: char) -> Option<Self>;
}

/// Trait for anything that wants to automatically wrap a `Token`.
pub trait TokenWrapper<T>
where
    Self: Sized,
    T: TokenTrait,
{
    /// Function that wraps the `Token` and the `Context` and returns itself.
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

/// Trait for anything that wants to match a single `Token`.
pub trait TokenMatch<T>
where
    T: TokenTrait,
{
    /// Function that matches a single `Token`.
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

impl<T> TokenMatch<T> for Vec<T>
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
