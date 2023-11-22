mod cursor;
mod messages;
mod types;

use crate::messages::{get_tree, run_command};
use std::default::Default;
use std::os::unix::net::UnixStream;

use anyhow::Result;
use cursor::find_focused;
use types::Layout;

fn main() -> Result<()> {
    let mut socket = UnixStream::connect("/run/user/1000/sway-ipc.1000.2653.sock")?;

    let tree = get_tree(&mut socket)?;
    match find_focused(&tree) {
        None => println!("on focused tiles node"),
        Some(f) => {
            let mut c = f;
            let mut nav: String = Default::default();

            while let Ok(p) = c.ascend() {
                if p.focus.layout == Layout::SplitH
                    || p.focus.layout == Layout::SplitV
                    || p.focus.layout == Layout::Output
                {
                    break;
                };
                nav += if nav.is_empty() { "" } else { "; " };
                nav += "focus parent";
                c = p;
            }

            nav += if nav.is_empty() { "" } else { "; " };
            nav += "focus left";

            match run_command(&mut socket, &nav) {
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
