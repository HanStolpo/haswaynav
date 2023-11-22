use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, Copy, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum SwayNodeType {
    #[default]
    Root,
    Output,
    Workspace,
    Con,
    FloatingCon,
}

#[test]
fn test_sway_node_type_deserialize() {
    let json = r#"[ "root" , "output" , "workspace" , "con" , "floating_con" ]"#;

    let expected = {
        use SwayNodeType::*;
        [Root, Output, Workspace, Con, FloatingCon]
    };

    let parsed: Vec<SwayNodeType> = serde_json::from_str(json).unwrap();

    assert_eq!(parsed.as_ref(), expected);
}

#[derive(Deserialize, Debug, PartialEq, Eq, Copy, Clone, Default)]
#[serde(rename_all = "snake_case")]
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
pub enum ApplicationInhibitor {
    None,
    Enabled,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum UserInhibitor {
    None,
    Focus,
    Fullscreen,
    Open,
    Visible,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct InhibitorState {
    pub application: ApplicationInhibitor,
    pub user: UserInhibitor,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(dead_code)]
pub struct SwayTreeNode {
    pub id: i32,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub node_type: SwayNodeType,
    pub current_border_width: i32,
    pub layout: Layout,
    pub orientation: Orientation,
    pub percent: Option<f32>,
    pub rect: Rect,
    pub window_rect: Rect,
    pub deco_rect: Rect,
    pub geometry: Rect,
    pub urgent: bool,
    pub sticky: bool,
    pub marks: Vec<String>,
    pub focused: bool,
    pub focus: Vec<i32>,
    pub nodes: Vec<SwayTreeNode>,
    pub floating_nodes: Vec<SwayTreeNode>,
    pub representation: Option<String>,
    pub fullscreen_mode: FullScreenMode,
    pub app_id: Option<String>,
    pub pid: Option<i32>,
    pub visible: Option<bool>,
    pub shell: Option<String>,
    pub inhibit_idle: Option<bool>,
    pub idle_inhibitors: Option<InhibitorState>,
}

#[test]
fn test_sway_tree_node_deserialize() {
    let example = include_str!("types/sway-tree.json");

    let parsed: Result<SwayTreeNode, serde_json::Error> = serde_json::from_str(example);

    match parsed {
        Ok(_) => (),
        Err(err) => panic!("error decoding example json: {:?}", err),
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct CommandResult {
    pub success: bool,
    pub parse_error: Option<bool>,
    pub error: Option<String>,
}
