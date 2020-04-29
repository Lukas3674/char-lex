/// Context type for a `Token` with the [`TokenTrait`].
///
/// [`TokenTrait`]: https://doc.rust-lang.org/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Context {
    /// The corresponding [`char`] in this [`Context`].
    ///
    /// [`char`]: https://doc.rust-lang.org/std/primitive.char.html
    /// [`Context`]: https://doc.rust-lang.org/
    pub character: char,

    /// The corresponding `position` in this [`Context`].
    ///
    /// It is meant to represent `(line, character)` positions.
    ///
    /// [`Context`]: https://doc.rust-lang.org/
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
