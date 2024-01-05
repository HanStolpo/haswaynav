use clap::Parser;
use haswaynav::{
    change_focus,
    cli::{Commands, FocusArgs},
    sway_connect,
};

use anyhow::Result;

fn main() -> Result<()> {
    let command = Commands::parse();

    let mut socket = sway_connect()?;

    match command {
        Commands::Focus(FocusArgs { direction }) => change_focus(&mut socket, direction)?,
    }

    Ok(())
}
