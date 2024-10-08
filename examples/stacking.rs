//! This example shows how to use the [`Stacked`] widget.

use tuit::prelude::*;
use tuit::std::stdout_render::StdoutRenderer;
use tuit::style::{Ansi4, Colour, Style};
use tuit::terminal::ConstantSize;
use tuit::widgets::builtins::{Sweeper, Text, WithLayout};

fn main() {
    let sweeper = Sweeper::of_colour(Colour::Ansi16(Ansi4::Cyan));

    let top_text    = Text::new("Top widget");
    let middle_text = Text::new("Middle widget");
    let bottom_text = Text::new("Bottom widget").styled(Style::new().bg(Colour::Ansi16(Ansi4::Red)));

    let stacked = top_text
        .on_top_of(middle_text)
        .on_top_of(bottom_text)
        .centered();

    let mut terminal: ConstantSize<30, 9> = ConstantSize::new();

    sweeper.drawn(&mut terminal).expect("Infallible");
    stacked.drawn(&mut terminal).expect("Infallible");

    terminal.display(StdoutRenderer::default()).expect("Should be Ok");
}