//! All the currently supported messages which can be sent to sway over its domain socket.

use std::os::unix::net::UnixStream;

use crate::tree::{CommandResult, TreeNode};
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::io::Read;
use std::io::Write;

const MAGIC_BYTES: [u8; 6] = *(b"i3-ipc");

#[derive(Copy, Clone)]
/// The identifier for the sway message being sent via IPC
enum MessageType {
    RunCommand = 0,
    GetTree = 4,
}

/// Send a message over the IPC socket to sway
fn send_message(sock: &mut UnixStream, message_type: MessageType, payload: &[u8]) -> Result<()> {
    sock.write_all(&MAGIC_BYTES)?;

    let payload_length: i32 = payload.len().try_into()?;
    sock.write_all(&(payload_length).to_ne_bytes())?;

    sock.write_all(&(message_type as i32).to_ne_bytes())?;

    sock.write_all(payload)?;

    sock.flush()?;

    Ok(())
}

/// Receive a response over the IPC socket from sway after sending a message
fn receive_message<T: DeserializeOwned>(
    sock: &mut UnixStream,
    message_type: MessageType,
) -> Result<T> {
    let mut magic_bytes: [u8; 6] = *(b"000000");
    sock.read_exact(&mut magic_bytes)
        .context("reading magic bytes")?;
    if magic_bytes != MAGIC_BYTES {
        anyhow::bail!(
            "expected {:?} as magic bytes but got {:?}",
            &MAGIC_BYTES,
            &magic_bytes
        );
    }

    let payload_length = {
        let mut payload_length_bytes = (0 as i32).to_ne_bytes();
        sock.read_exact(&mut payload_length_bytes)
            .context("reading payload length")?;
        i32::from_ne_bytes(payload_length_bytes)
    };

    let payload_type = {
        let mut bytes = (0 as i32).to_ne_bytes();
        sock.read_exact(&mut bytes).context("payload type")?;
        i32::from_ne_bytes(bytes)
    };
    if payload_type != message_type as i32 {
        anyhow::bail!(
            "Wrong payload type specifier, expected {} but got {}",
            message_type as i32,
            payload_type
        );
    };

    let payload_json: Vec<u8> = {
        let mut payload = vec![0; payload_length as usize];
        sock.read_exact(&mut payload).context("reading payload")?;
        payload
    };

    let payload = serde_json::from_slice(&payload_json).context("decoding payload")?;

    Ok(payload)
}

/// Send a message to sway over the IPC socket and then receive its response to the message.
fn message<T: DeserializeOwned>(
    sock: &mut UnixStream,
    message_type: MessageType,
    payload: &[u8],
) -> Result<T> {
    send_message(sock, message_type, payload)?;
    Ok(receive_message(sock, message_type)?)
}

/// Get the node layout tree by sending a `GET_TREE` message to sway over the IPC socket.
pub fn get_tree(sock: &mut UnixStream) -> Result<TreeNode> {
    Ok(message(sock, MessageType::GetTree, &[])?)
}

/// Run the supplied string as sway commands by sending the `RUN_COMMAND` message to sway over the
/// IPC socket.
pub fn run_command(sock: &mut UnixStream, commands: &str) -> Result<Vec<CommandResult>> {
    Ok(message(sock, MessageType::RunCommand, commands.as_bytes())?)
}
