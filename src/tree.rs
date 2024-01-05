//! Data types for representing sways layout as a tree.

use serde::Deserialize;

pub mod cursor;

#[derive(Deserialize, Debug, PartialEq, Eq, Copy, Clone, Default)]
#[serde(rename_all = "snake_case")]
/// See [TreeNode::node_type]
pub enum NodeType {
    #[default]
    Root,
    Output,
    Workspace,
    Con,
    FloatingCon,
}

#[test]
fn test_node_type_deserialize() {
    let json = r#"[ "root" , "output" , "workspace" , "con" , "floating_con" ]"#;

    let expected = {
        use NodeType::*;
        [Root, Output, Workspace, Con, FloatingCon]
    };

    let parsed: Vec<NodeType> = serde_json::from_str(json).unwrap();

    assert_eq!(parsed.as_ref(), expected);
}

#[derive(Deserialize, Debug, PartialEq, Eq, Copy, Clone, Default)]
#[serde(rename_all = "snake_case")]
/// See [TreeNode::border]
pub enum Border {
    #[default]
    None,
    Normal,
    Pixel,
    Csd,
}

#[test]
fn test_border_deserialize() {
    let json = r#"[ "normal" , "none" , "pixel" , "csd"]"#;

    let expected = {
        use Border::*;
        [Normal, None, Pixel, Csd]
    };

    let parsed: Vec<Border> = serde_json::from_str(json).unwrap();

    assert_eq!(parsed.as_ref(), expected);
}

#[derive(Deserialize, Debug, PartialEq, Eq, Copy, Clone, Default)]
#[serde(rename_all = "lowercase")]
/// See [TreeNode::layout]
pub enum Layout {
    #[default]
    None, // realworld example uses none for views
    SplitH,
    SplitV,
    Stacked,
    Tabbed,
    Output,
}

#[test]
fn test_layout_deserialize() {
    let json = r#"["none", "splith", "splitv", "stacked", "tabbed", "output"]"#;

    let expected = {
        use Layout::*;
        [None, SplitH, SplitV, Stacked, Tabbed, Output]
    };

    let parsed: Vec<Layout> = serde_json::from_str(json).unwrap();

    assert_eq!(parsed.as_ref(), expected);
}

#[derive(Deserialize, Debug, PartialEq, Eq, Copy, Clone, Default)]
#[serde(rename_all = "lowercase")]
/// See [TreeNode::orientation]
pub enum Orientation {
    #[default]
    None,
    Vertical,
    Horizontal,
}

#[test]
fn test_orientation_deserialize() {
    let json = r#"["vertical", "horizontal", "none"]"#;

    let expected = {
        use Orientation::*;
        [Vertical, Horizontal, None]
    };

    let parsed: Vec<Orientation> = serde_json::from_str(json).unwrap();

    assert_eq!(parsed.as_ref(), expected);
}

#[derive(Deserialize, Debug, PartialEq, Eq, Copy, Clone, Default)]
/// The definition of a rectangle returned from sway to describe geometries
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[test]
fn test_rect_deserialize() {
    let json = r#"{"x": 0, "y": 1, "height": 2, "width": 3}"#;

    let expected = Rect {
        x: 0,
        y: 1,
        height: 2,
        width: 3,
    };

    let parsed: Rect = serde_json::from_str(json).unwrap();

    assert_eq!(parsed, expected);
}

#[derive(Deserialize, Debug, PartialEq, Eq, Copy, Clone, Default)]
#[serde(try_from = "i32")]
/// See [TreeNode::fullscreen_mode]
pub enum FullScreenMode {
    #[default]
    None,
    FullWorkspace,
    GlobalFullScreen,
}

impl TryFrom<i32> for FullScreenMode {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use FullScreenMode::*;
        match value {
            0 => Ok(None),
            1 => Ok(FullWorkspace),
            2 => Ok(GlobalFullScreen),
            _ => Err(format!(
                "Unexpected integer ID {} expecting 0, 1 or 2.",
                value
            )),
        }
    }
}

#[test]
fn test_full_screen_mode_deserialize() {
    let json = r#"[0, 1, 2]"#;

    let expected = {
        use FullScreenMode::*;
        [None, FullWorkspace, GlobalFullScreen]
    };

    let parsed: Vec<FullScreenMode> = serde_json::from_str(json).unwrap();

    assert_eq!(parsed.as_ref(), expected);
}

#[derive(Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
/// See [TreeNode::idle_inhibitors]
pub enum ApplicationInhibitor {
    None,
    Enabled,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
/// See [TreeNode::idle_inhibitors]
pub enum UserInhibitor {
    None,
    Focus,
    Fullscreen,
    Open,
    Visible,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
/// See [TreeNode::idle_inhibitors]
pub struct InhibitorState {
    pub application: ApplicationInhibitor,
    pub user: UserInhibitor,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(dead_code)]
/// The structure returned by the sway IPC `GET_TREE` message, see `man sway-ipc`.
pub struct TreeNode {
    /// The internal unique ID for this node
    pub id: i32,
    /// The name of the node such as the output name or window title. For the scratchpad, this will be __i3_scratch for compatibility with i3.
    pub name: Option<String>,
    #[serde(rename = "type")]
    /// The node type. It can be root, output, workspace, con, or floating_con
    pub node_type: NodeType,
    /// The border style for the node. It can be normal, none, pixel, or csd
    pub border: Border,
    /// Number of pixels used for the border width
    pub current_border_width: i32,
    /// The node's layout. It can either be splith, splitv, stacked, tabbed, or output
    pub layout: Layout,
    /// The node's orientation. It can be vertical, horizontal, or none
    pub orientation: Orientation,
    /// The percentage of the node's parent that it takes up or null for the root and other special nodes such as the scratchpad
    pub percent: Option<f32>,
    /// The absolute geometry of the node. The window decorations are excluded from this, but borders are included.
    pub rect: Rect,
    /// The geometry of the content inside the node. These coordinates are relative to the node itself. Window decorations and borders are outside the window_rect
    pub window_rect: Rect,
    /// The geometry of the decorations for the node relative to the parent node
    pub deco_rect: Rect,
    /// The natural geometry of the contents if it were to size itself
    pub geometry: Rect,
    /// Whether the node or any of its descendants has the urgent hint set. Note: This may not exist when compiled without xwayland support
    pub urgent: bool,
    /// Whether the node is sticky (shows on all workspaces)
    pub sticky: bool,
    /// List of marks assigned to the node
    pub marks: Vec<String>,
    /// Whether the node is currently focused by the default seat (seat0)
    pub focused: bool,
    /// Array of child node IDs in the current focus order
    pub focus: Vec<i32>,
    /// The tiling children nodes for the node
    pub nodes: Vec<TreeNode>,
    /// The floating children nodes for the node
    pub floating_nodes: Vec<TreeNode>,
    /// (Only workspaces) A string representation of the layout of the workspace that can be used as an aid in submitting reproduction steps for bug reports
    pub representation: Option<String>,
    /// (Only containers and views) The fullscreen mode of the node. 0 means none, 1 means full workspace, and 2 means global fullscreen
    pub fullscreen_mode: FullScreenMode,
    /// (Only views) For an xdg-shell view, the name of the application, if set. Otherwise, null
    pub app_id: Option<String>,
    /// (Only views) The PID of the application that owns the view
    pub pid: Option<i32>,
    /// (Only views) Whether the node is visible
    pub visible: Option<bool>,
    /// (Only views) The shell of the view, such as xdg_shell or xwayland
    pub shell: Option<String>,
    /// (Only views) Whether the view is inhibiting the idle state
    pub inhibit_idle: Option<bool>,
    /// (Only views) An object containing the state of the application and user idle inhibitors. application can be enabled or none. user can be focus, fullscreen, open, visible or none.
    pub idle_inhibitors: Option<InhibitorState>,
}

impl<'a> IntoIterator for &'a TreeNode {
    type Item = cursor::Cursor<'a>;
    type IntoIter = cursor::CursorIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        cursor::Cursor::new(self).iter()
    }
}

#[test]
fn test_tree_node_deserialize() {
    let example = include_str!("tree/sway-tree.json");

    let parsed: Result<TreeNode, serde_json::Error> = serde_json::from_str(example);

    match parsed {
        Ok(_) => (),
        Err(err) => panic!("error decoding example json: {:?}", err),
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
/// The reply received when sending the `RUN_COMMAND` sway IPC message, see `man sway-ipc`.
pub struct CommandResult {
    /// A boolean indacting whether the command was successful
    pub success: bool,
    /// True when the command failed because the command was uknown or could not be parsed.
    pub parse_error: Option<bool>,
    /// A human readable error message in case of failure
    pub error: Option<String>,
}
