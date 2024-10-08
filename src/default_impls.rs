use core::cmp::Ordering;
use core::ops::{BitOr, Deref, DerefMut};
use crate::draw::Renderer;
use crate::prelude::{Metadata, Terminal, TerminalConst, TerminalMut};
use crate::style::{Ansi4, Style};
use crate::terminal::{Cell, UpdateInfo, UpdateResult};
use crate::terminal::Rectangle;
use crate::widgets::Widget;

impl BitOr for Ansi4 {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u8) << 4 | rhs as u8
    }
}
impl From<Rectangle> for ((usize, usize), (usize, usize)) {
    fn from(value: Rectangle) -> Self {
        (value.left_top(), value.right_bottom())
    }
}
impl From<((usize, usize), (usize, usize))> for Rectangle {
    fn from((left_top, right_bottom): ((usize, usize), (usize, usize))) -> Self {
        Self::new(left_top, right_bottom)
    }
}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.area().cmp(&other.area()))
    }
}
impl Ord for Rectangle {
    fn cmp(&self, other: &Self) -> Ordering {
        #[allow(clippy::unwrap_used)]
        self.partial_cmp(other).unwrap()
    }
}

impl<T: Deref<Target: Metadata>> Metadata for T {
    fn dimensions(&self) -> (usize, usize) {
        self.deref().dimensions()
    }

    fn default_style(&self) -> Style {
        self.deref().default_style()
    }
}
impl<T: Deref<Target: TerminalConst>> TerminalConst for T {
    fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.deref().cells()
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.deref().cell(x, y)
    }
}
impl<T: DerefMut<Target: TerminalMut>> TerminalMut for T {
    fn cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.deref_mut().cells_mut()
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.deref_mut().cell_mut(x, y)
    }
}
impl<T: DerefMut<Target: Renderer>> Renderer for T {
    fn render(&mut self, terminal: impl TerminalConst) -> crate::Result<()> {
        self.deref_mut().render(terminal)
    }
}

impl<T: DerefMut<Target: Widget>> Widget for T {
    fn update(&mut self, update_info: UpdateInfo, terminal: impl TerminalConst) -> crate::Result<UpdateResult> {
        self.deref_mut().update(update_info, terminal)
    }

    fn draw(&self, terminal: impl Terminal) -> crate::Result<UpdateResult> {
        self.deref().draw(terminal)
    }
}

impl<T: TerminalConst + TerminalMut + Metadata> Terminal for T {}