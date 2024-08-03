use crate::Error;
use crate::prelude::{Terminal, TerminalConst, Widget};
use crate::style::Style;
use crate::terminal::{UpdateInfo, UpdateResult, Rectangle};
use crate::widgets::{BoundingBox, };

/// Text at the top-left of the terminal.
pub struct Text<'a> {
    /// The text to display.
    pub text: &'a str,
    /// The style with which to display it.
    pub style: Style
}

impl<'a> Text<'a> {
    /// Create a new [`Text`] with the default style.
    ///
    /// ## Create a new [`Text`]
    ///
    /// ```
    /// use tuit::style::Style;
    /// use tuit::widgets::builtins::Text;
    ///
    /// let text_widget = Text::new("Hello!");
    ///
    /// assert_eq!(text_widget.text, "Hello!");
    /// assert_eq!(text_widget.style, Style::new());
    /// ```
    ///
    /// ## Using [`Text`]
    ///
    /// ```
    /// use tuit::prelude::*;
    /// use tuit::widgets::builtins::Text;
    /// use tuit::terminal::ConstantSize;
    ///
    /// // 20x20 cells terminal. Okay... maybe not tremendous, but... you get it, right?
    /// let mut tremendous_terminal: ConstantSize<20, 20> = ConstantSize::new();
    /// // 1x3 cells terminal
    /// let mut tiny_terminal: ConstantSize<1, 3> = ConstantSize::new();
    ///
    /// let text_widget = Text::new("Hello!");
    ///
    /// text_widget.drawn(&mut tremendous_terminal).expect("There is enough space");
    /// text_widget.drawn(&mut tiny_terminal).expect_err("There is not enough space, so we get an `Err`.");
    /// ```
    #[must_use]
    pub const fn new(text: &'a str) -> Self {
        Self {
            text,
            style: Style::new()
        }
    }

    /// Apply a [`Style`] to the [`Text`].
    ///
    /// ```
    /// use tuit::style::{Ansi4, Style};
    /// use tuit::widgets::builtins::Text;
    ///
    /// let style = Style::new().bg_ansi4(Ansi4::BrightCyan);
    /// let text_widget = Text::new("Hello!").styled(style);
    ///
    /// assert_eq!(text_widget.text, "Hello!");
    /// assert_eq!(text_widget.style, style);
    /// ```
    #[must_use]
    pub const fn styled(mut self, style: Style) -> Self{
        self.style = style;

        self
    }
}

impl Widget for Text<'_> {
    fn update(&mut self, _update_info: UpdateInfo, _terminal: impl TerminalConst) -> crate::Result<UpdateResult> {
        Ok(UpdateResult::NoEvent)
    }

    fn draw(&self, _update_info: UpdateInfo, mut terminal: impl Terminal) -> crate::Result<UpdateResult> {
        let mut cells = terminal.cells_mut();

        for (idx, character) in self.text.chars().enumerate() {
            let current_cell = cells
                .next()
                .ok_or(Error::OutOfBoundsCharacter(idx))?;

            current_cell.character = character;
            current_cell.style = self.style;
        }

        Ok(UpdateResult::NoEvent)
    }
}

impl BoundingBox for Text<'_> {
    fn bounding_box(&self, terminal: impl TerminalConst) -> Rectangle {
        let height = self.text.len() / terminal.width();
        let width = self.text.len().min(terminal.width());

        Rectangle::of_size(width, height)
    }

    fn completely_covers(&self, rectangle: Rectangle) -> bool {
        self.text.len() >= rectangle.area()
    }
}