//! All the types related to the CLI

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(long_about= None)]
/// Custom navigation commands for sway
pub enum Commands {
    #[command(name = "focus")]
    /// Perform a change of focus in the given direction skipping over tabbed and stacked siblings.
    Focus(FocusArgs),
}

#[derive(Debug, clap::Args)]
/// The only arguments to the focus command is the direction
pub struct FocusArgs {
    pub direction: Direction,
}

#[derive(Debug, clap::ValueEnum, Clone)]
/// The enumeration of directions used with focus to change focus in a specified direction.
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
