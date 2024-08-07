use crate::prelude::*;
use crate::style::Style;
use crate::terminal::{MouseButton, Rectangle, UpdateInfo, UpdateResult};
use crate::widgets::BoundingBox;
use crate::widgets::builtins::Text;

/// A prompt that is centered
///
/// ```
/// use tuit::widgets::Widget;
/// use tuit::terminal::ConstantSize;
/// use tuit::widgets::builtins::CenteredText;
///
/// let small_terminal: ConstantSize<1, 1> = ConstantSize::new();
/// let my_prompt = CenteredText::new("Hello world!");
///
/// my_prompt.drawn(small_terminal).expect_err("If the terminal is too small, then an OutOfBoundsCoordinate error is returned.");
/// ```
#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct CenteredText<'a> {
    /// The text to be displayed
    pub prompt_text: &'a str,
    /// The styling behind the prompt.
    pub style: Style,
}

impl<'a> CenteredText<'a> {
    /// Initializes a [`CenteredText`] with the given text.
    ///
    /// ```
    /// use tuit::terminal::ConstantSize;
    /// use tuit::widgets::builtins::CenteredText;
    /// use tuit::prelude::*;
    ///
    /// let mut my_terminal = ConstantSize::<20, 20>::new();
    /// let my_text = CenteredText::new("Hello!");
    ///
    /// my_text
    ///     .drawn(&mut my_terminal)
    ///     .expect("This only fails if the prompt text has more characters than the terminal can contain.");
    /// ```
    #[must_use]
    pub const fn new(text: &'a str) -> Self {
        Self {
            prompt_text: text,
            style: Style::new(),
        }
    }

    /// Initializes a [`CenteredText`] with the given text.
    ///
    /// ```
    /// use tuit::terminal::{ConstantSize};
    /// use tuit::style::{Ansi4, Style};
    /// use tuit::widgets::builtins::CenteredText;
    /// use tuit::prelude::*;
    ///
    /// let mut my_terminal = ConstantSize::<20, 20>::new();
    /// let my_style = Style::new().bg_ansi4(Ansi4::Blue);
    /// let my_text = CenteredText::new("Hello!").style(my_style);
    ///
    /// my_text
    ///     .drawn(&mut my_terminal)
    ///     .expect("This only fails if the prompt text has more characters than the terminal can contain.");
    /// ```
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;

        self
    }
}

impl<'a> Widget for CenteredText<'a> {
    fn update(
        &mut self,
        update_info: UpdateInfo,
        terminal: impl TerminalConst,
    ) -> crate::Result<UpdateResult> {
        match update_info {
            UpdateInfo::CellClicked(x, y, MouseButton::LeftClick) => {
                if self.bounding_box(terminal.bounding_box())?.contains((x, y)) {
                    return Ok(UpdateResult::Interacted)
                }

                Ok(UpdateResult::NoEvent)
            }
            _ => Ok(UpdateResult::NoEvent),
        }
    }

    fn draw(
        &self,
        _update_info: UpdateInfo,
        mut terminal: impl Terminal,
    ) -> crate::Result<UpdateResult> {
        let view = terminal
            .view_mut(self.bounding_box(terminal.bounding_box())?)
            .expect("CenteredText should always produce a valid bounding box");

        let text = Text::new(self.prompt_text).styled(self.style);

        text.drawn(view)?;

        Ok(UpdateResult::NoEvent)
    }
}

impl BoundingBox for CenteredText<'_> {
    fn bounding_box(&self, rect: Rectangle) -> crate::Result<Rectangle> {
        let (terminal_width, terminal_height) = rect.dimensions();

        let text_len = self.prompt_text.len();
        // Calculate the width/height of the prompt, capping it to the terminal's width.
        //    // `div_ceil` because if the terminal width is 12, and the text length is 13,
        //    // we want the height to be 2 because it takes 2 lines.
        let height = text_len.div_ceil(terminal_width).min(terminal_height);
        let width = text_len.min(terminal_width);

        let horizontal_center = terminal_width / 2;
        let vertical_center = terminal_height / 2;

        let left = horizontal_center - (width / 2);
        let right = left + width;

        let top = vertical_center - (height / 2);
        let bottom = top + height;

        Ok(Rectangle::new((left, top), (right, bottom)))
    }

    fn completely_covers(&self, rectangle: Rectangle) -> bool {
        rectangle.area() <= self.prompt_text.len()
    }
}