use crate::prelude::*;
use crate::style::Style;
use crate::terminal::Cell;
#[allow(unused_imports)]
// is used for rustdoc.
use crate::terminal::ConstantSize;

/// An implementation of the [`ConstantSize`] that can be created from mutable references to arrays
/// or even [`alloc::boxed::Box`] if your terminal's cells are too big to fit on the stack.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct ConstantSizeRef<const WIDTH: usize, const HEIGHT: usize, T> {
    // Modifying this does not lead to UB, so they are public.
    /// The characters that are within the terminal.
    pub characters: T,
    /// The terminal's default style.
    pub default_style: Style,
}

// That certainly is a mouthful...
impl<const WIDTH: usize, const HEIGHT: usize, T> ConstantSizeRef<WIDTH, HEIGHT, T> {
    /// Creates a new [`ConstantSizeRef`] with the specified terminal
    #[must_use]
    pub const fn new(terminal: T) -> Self {
        Self {
            characters: terminal,
            default_style: Style::new(),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T> Metadata for ConstantSizeRef<WIDTH, HEIGHT, T>
{
    fn dimensions(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }

    fn default_style(&self) -> Style {
        self.default_style
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T> TerminalConst for ConstantSizeRef<WIDTH, HEIGHT, T>
where
    T: AsRef<[[Cell; WIDTH]; HEIGHT]>,
{
    fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.characters.as_ref().iter().flatten()
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        let row = self.characters.as_ref().get(y)?;

        row.get(x)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T> TerminalMut for ConstantSizeRef<WIDTH, HEIGHT, T>
where
    T: AsMut<[[Cell; WIDTH]; HEIGHT]> + AsRef<[[Cell; WIDTH]; HEIGHT]>,
{
    fn cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.characters.as_mut().iter_mut().flatten()
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let row = self.characters.as_mut().get_mut(y)?;

        row.get_mut(x)
    }
}
