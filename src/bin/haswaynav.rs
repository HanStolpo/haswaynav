use std::os::unix::net::UnixStream;
use haswaynav::messages::{get_tree, run_command};

use anyhow::Result;
use clap::Parser;
use haswaynav::tree::cursor::find_focused;
use haswaynav::tree::Layout;

#[derive(Debug, Parser)]
#[clap(long_about= None)]
enum Args {
    #[command(name = "focus")]
    Focus(FocusArgs),
}

#[derive(Debug, clap::Args)]
struct FocusArgs {
    direction: Direction,
}

#[derive(Debug, clap::ValueEnum, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut socket = UnixStream::connect("/run/user/1000/sway-ipc.1000.2653.sock")?;

    match args {
        Args::Focus(FocusArgs { direction }) => change_focus(&mut socket, direction)?,
    }

    Ok(())
}

fn change_focus(socket: &mut UnixStream, dir: Direction) -> Result<()> {
    let tree = get_tree(socket)?;
    let focus_dir = match dir {
        Direction::Left => "focus left",
        Direction::Right => "focus right",
        Direction::Up => "focus up",
        Direction::Down => "focus down",
    };
    match find_focused(&tree) {
        None => println!("on focused tiles node"),
        Some(c) => {
            let nav = c
                .ancestors()
                .map_while(|x| {
                    if x.get_node().layout == Layout::SplitH
                        || x.get_node().layout == Layout::SplitV
                        || x.get_node().layout == Layout::Output
                    {
                        None
                    } else {
                        Some("focus parent")
                    }
                })
                .chain([focus_dir])
                .collect::<Vec<_>>()
                .join("; ");

            match run_command(socket, &nav) {
                Err(err) => anyhow::bail!("Failed running navigation command: {}", err),
                Ok(xs) => {
                    for x in xs {
                        if !x.success {
                            anyhow::bail!("Failure reported by sway: {:?}", x.error)
                        }
                    }
                }
            }
        }
    };

    Ok(())
}