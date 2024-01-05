//! A small utility that modifies the way one navigates in sway where `focus left` etc skips over
//! siblings in a tabbed or stacked containers.
//!
//! Normally when you change the focus horizontally and the focussed window is in a tabbed
//! container then the nexted tabbed sibling will be selected instead of next container physically
//! in the specified direction. This little utility allows you to select the next container in the
//! physical direction.

use std::os::unix::net::UnixStream;

pub mod cli;
pub mod messages;
pub mod tree;

use anyhow::Result;
use cli::Direction;
use messages::{get_tree, run_command};
use tree::{cursor::find_focused, Layout};

/// Read the path to the sway domain socket from the `SWAYSOCK` environment variable and connect to it
/// returning a descriptive error message if any error occurs.
pub fn sway_connect() -> Result<UnixStream> {
    let swayswock = {
        let errmsg = || {
            anyhow::format_err!("Environment variable 'SWAYSOCK' which specifies the path to the sway socket is not defined")
        };
        std::env::var("SWAYSOCK")
            .map_err(|_| errmsg())
            .and_then(|s| if s.is_empty() { Err(errmsg()) } else { Ok(s) })?
    };

    UnixStream::connect(swayswock.clone()).map_err(|err| {
        anyhow::format_err!(
            "Failed opening socket '{}' specified by SWAYSOCK environment variable: {}",
            swayswock,
            err
        )
    })
}

/// Change the focus to the next visible window in the specified direction. This will ignore the
/// other siblings in a tabbed or stacked container.
pub fn change_focus(socket: &mut UnixStream, dir: Direction) -> Result<()> {
    let tree = get_tree(socket)?;
    let focus_dir = match dir {
        Direction::Left => "focus left",
        Direction::Right => "focus right",
        Direction::Up => "focus up",
        Direction::Down => "focus down",
    };
    match find_focused(&tree) {
        None => println!("no focused node"),
        Some(c) => {
            let nav = c
                .ancestors()
                .into_iter()
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
