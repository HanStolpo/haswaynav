use std::default::Default;

use crate::types::SwayTreeNode;

pub fn find_focused<'a>(root: &'a SwayTreeNode) -> Option<Cursor<'a>> {
    CursorIterator::new(Cursor::new(&root)).find(|c| c.focus.focused)
}

#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    parent_stack: Vec<Cursor<'a>>,
    pub focus: &'a SwayTreeNode,
    focus_idx: usize,
}

impl<'a> Cursor<'a> {
    #[allow(dead_code)]
    pub fn new(focus: &'a SwayTreeNode) -> Cursor<'a> {
        Cursor {
            focus,
            parent_stack: Default::default(),
            focus_idx: 0,
        }
    }

    fn left_most_descendant(mut self) -> Self {
        loop {
            self = match self.descend() {
                Result::Err(c) => return c,
                Result::Ok(c) => c,
            }
        }
    }

    pub fn descend(mut self: Self) -> Result<Self, Self> {
        if self.focus.nodes.is_empty() {
            Result::Err(self)
        } else {
            self.parent_stack.push(self.clone());
            self.focus = &self.focus.nodes[0];
            self.focus_idx = 0;
            Result::Ok(self)
        }
    }

    #[allow(dead_code)]
    pub fn prev_sibling(mut self: Self) -> Result<Self, Self> {
        let parent = match self.parent_stack.last() {
            None => return Result::Err(self),
            Some(x) => x,
        };
        if self.focus_idx == 0 {
            Result::Err(self)
        } else {
            self.focus_idx -= 1;
            self.focus = &parent.focus.nodes[self.focus_idx];
            Result::Ok(self)
        }
    }

    pub fn next_sibling(mut self: Self) -> Result<Self, Self> {
        let parent = match self.parent_stack.last() {
            None => return Result::Err(self),
            Some(x) => x,
        };
        if self.focus_idx + 1 == parent.focus.nodes.len() {
            Result::Err(self)
        } else {
            self.focus_idx += 1;
            self.focus = &parent.focus.nodes[self.focus_idx];
            Result::Ok(self)
        }
    }

    pub fn ascend(mut self: Self) -> Result<Self, Self> {
        match self.parent_stack.pop() {
            None => Result::Err(self),
            Some(x) => Result::Ok(x),
        }
    }
}

#[derive(Debug)]
struct CursorIterator<'a>(Result<Cursor<'a>, Cursor<'a>>);

impl<'a> CursorIterator<'a> {
    #[allow(dead_code)]
    pub fn new(c: Cursor<'a>) -> Self {
        CursorIterator(Result::Ok(Cursor::left_most_descendant(c)))
    }
}

impl<'a> std::iter::Iterator for CursorIterator<'a> {
    type Item = Cursor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut c = match self {
            CursorIterator(Result::Err(_)) => return None,
            CursorIterator(Result::Ok(c)) => match c.clone().next_sibling() {
                Result::Err(c) => c.ascend(),
                Result::Ok(c) => Result::Ok(Cursor::left_most_descendant(c)),
            },
        };

        std::mem::swap(&mut self.0, &mut c);

        match c {
            Result::Err(_) => None,
            Result::Ok(c) => Some(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree() -> SwayTreeNode {
        SwayTreeNode {
            name: Some("a".to_string()),
            nodes: vec![SwayTreeNode {
                name: Some("b".to_string()),

                nodes: vec![
                    SwayTreeNode {
                        name: Some("c".to_string()),
                        ..Default::default()
                    },
                    SwayTreeNode {
                        name: Some("d".to_string()),

                        nodes: vec![SwayTreeNode {
                            name: Some("e".to_string()),

                            nodes: vec![SwayTreeNode {
                                name: Some("f".to_string()),
                                focused: true,
                                ..Default::default()
                            }],
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    SwayTreeNode {
                        name: Some("g".to_string()),

                        nodes: vec![SwayTreeNode {
                            name: Some("h".to_string()),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        }
    }

    mod cursor {
        use super::*;

        #[test]
        fn navigation_descend() {
            let tree = build_tree();
            let res: Result<Cursor, Cursor> = (|| Cursor::new(&tree).descend()?.descend())();
            assert_eq!(
                "c",
                &res.unwrap().focus.name.clone().unwrap_or("".to_string())
            );
        }

        #[test]
        fn navigation_next_sibling() {
            let tree = build_tree();
            let res: Result<Cursor, Cursor> =
                (|| Cursor::new(&tree).descend()?.descend()?.next_sibling())();
            assert_eq!(
                "d",
                &res.unwrap().focus.name.clone().unwrap_or("".to_string())
            );
        }

        #[test]
        fn navigation_prev_sibling() {
            let tree = build_tree();
            let res: Result<Cursor, Cursor> = (|| {
                Cursor::new(&tree)
                    .descend()?
                    .descend()?
                    .next_sibling()?
                    .prev_sibling()
            })();
            assert_eq!(
                "c",
                &res.unwrap().focus.name.clone().unwrap_or("".to_string())
            );
        }
    }

    mod iterator {
        use super::*;
        #[test]
        fn traversal() {
            let tree = build_tree();

            let names = CursorIterator::new(Cursor::new(&tree))
                .map(|c| c.focus.name.clone().unwrap_or("".to_string()))
                .collect::<Vec<String>>();

            assert_eq!(names.join(""), "cfedhgba".to_string());
        }

        #[test]
        fn find() {
            let tree = build_tree();
            let mut co = CursorIterator::new(Cursor::new(&tree)).find(|c| c.focus.focused);
            let mut trace: String = Default::default();
            while let Some(c) = co {
                trace += &c.focus.name.clone().unwrap_or("".to_string());
                co = c.ascend().ok();
            }
            assert_eq!(&trace, "fedba");
        }
    }
}
