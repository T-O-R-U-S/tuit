//! Show how split views can be used.

use tuit::prelude::*;
use tuit::std::stdout_render::StdoutRenderer;
use tuit::style::{Ansi4, Colour};
use tuit::terminal::{ConstantSize, ViewSplit};
use tuit::widgets::builtins::{Sweeper, Text};

fn main() {
    let mut terminal: ConstantSize<50, 20> = ConstantSize::new();

    let yellow = Sweeper::of_colour(Colour::Ansi16(Ansi4::Yellow));
    let magenta = Sweeper::of_colour(Colour::Ansi16(Ansi4::Magenta));
    let blue = Sweeper::of_colour(Colour::Ansi16(Ansi4::Blue)).with_shrink(2);

    let long_left_text = Text::new(
        "Here's some really long text that will probably, or at least I hope, wrap around when drawn on the left side of the terminal! It even has some extra padding to add space! Wow, isn't that cool!?"
    ).with_shrink(2);
    let short_right_text = Text::new("The guy next to me is too loud...").with_shrink(2);

    let mut split = ViewSplit::new(&mut terminal);

    let mut right = split.split_right_mut();
    yellow.drawn(&mut right).ok();
    blue.drawn(&mut right).ok();
    short_right_text.drawn(&mut right).ok();

    let mut left = split.split_left_mut();
    magenta.drawn(&mut left).ok();
    blue.drawn(&mut left).ok();
    long_left_text.drawn(&mut left).ok();

    terminal
        .display(StdoutRenderer::default())
        .expect("Can't fail");
}
