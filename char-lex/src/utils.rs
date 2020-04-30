/// Context type for a `Token`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Context {
    /// The corresponding `char` in this `Context`.
    pub character: char,

    /// The corresponding `position` in this `Context`.
    ///
    /// It is meant to represent as `(line, character)` position.
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
