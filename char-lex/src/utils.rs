/// Context type for a `Token` with the [`TokenTrait`].
///
/// [`TokenTrait`]: ../trait.TokenTrait.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Context {
    /// The corresponding [`char`] in this [`Context`].
    ///
    /// [`char`]: https://doc.rust-lang.org/std/primitive.char.html
    /// [`Context`]: ../utils/struct.Context.html
    pub character: char,

    /// The corresponding `position` in this [`Context`].
    ///
    /// It is meant to represent `(line, character)` positions.
    ///
    /// [`Context`]: ../utils/struct.Context.html
    pub position: (usize, usize),
}

impl Context {
    pub(super) fn new(character: char, position: (usize, usize)) -> Self {
        Self {
            character,
            position,
        }
    }
}
