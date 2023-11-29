use core::slice;
use std::{default::Default, iter::Rev};

use crate::tree::TreeNode;

pub fn find_focused<'a>(root: &'a TreeNode) -> Option<Cursor<'a>> {
    root.into_iter().find(|c| c.node.focused)
}

#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    parent_stack: Vec<Cursor<'a>>,
    node: &'a TreeNode,
    idx_in_parent: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(node: &'a TreeNode) -> Cursor<'a> {
        Cursor {
            node,
            parent_stack: Default::default(),
            idx_in_parent: 0,
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

    pub fn get_node(&self) -> &'a TreeNode {
        self.node
    }

    pub fn is_floating(&self) -> bool {
        match self.parent_stack.last() {
            None => false,
            Some(parent) => self.idx_in_parent >= parent.node.nodes.len(),
        }
    }

    pub fn ancestors(&self) -> Rev<slice::Iter<Self>> {
        self.parent_stack.iter().rev()
    }

    pub fn descend(mut self: Self) -> Result<Self, Self> {
        match self.deref_child(0) {
            None => Err(self),
            Some(child) => {
                self.parent_stack.push(self.clone());
                self.node = child;
                self.idx_in_parent = 0;
                Result::Ok(self)
            }
        }
    }

    fn deref_child(&self, mut idx: usize) -> Option<&'a TreeNode> {
        if idx < self.node.nodes.len() {
            Some(&self.node.nodes[idx])
        } else {
            idx -= self.node.nodes.len();
            if idx < self.node.floating_nodes.len() {
                Some(&self.node.floating_nodes[idx])
            } else {
                None
            }
        }
    }

    pub fn prev_sibling(mut self: Self) -> Result<Self, Self> {
        let parent = match self.parent_stack.last() {
            None => return Result::Err(self),
            Some(x) => x,
        };
        if self.idx_in_parent == 0 {
            Result::Err(self)
        } else {
            let new_idx = self.idx_in_parent - 1;
            match parent.deref_child(new_idx) {
                Some(new_focus) => {
                    self.node = new_focus;
                    self.idx_in_parent = new_idx;
                    Ok(self)
                }
                None => Err(self),
            }
        }
    }

    pub fn next_sibling(mut self: Self) -> Result<Self, Self> {
        let parent = match self.parent_stack.last() {
            None => return Result::Err(self),
            Some(x) => x,
        };
        let new_idx = self.idx_in_parent + 1;
        match parent.deref_child(new_idx) {
            Some(new_focus) => {
                self.node = new_focus;
                self.idx_in_parent = new_idx;
                Ok(self)
            }
            None => Err(self),
        }
    }

    pub fn ascend(mut self: Self) -> Result<Self, Self> {
        match self.parent_stack.pop() {
            None => Result::Err(self),
            Some(x) => Result::Ok(x),
        }
    }

    pub fn iter(self) -> CursorIterator<'a> {
        CursorIterator::new(self)
    }
}

impl<'a> IntoIterator for Cursor<'a> {
    type Item = Cursor<'a>;
    type IntoIter = CursorIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
pub struct CursorIterator<'a>(Result<Cursor<'a>, Cursor<'a>>);

impl<'a> CursorIterator<'a> {
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

    fn build_tree() -> TreeNode {
        TreeNode {
            name: Some("a".to_string()),
            nodes: vec![TreeNode {
                name: Some("b".to_string()),

                nodes: vec![
                    TreeNode {
                        name: Some("c".to_string()),
                        ..Default::default()
                    },
                    TreeNode {
                        name: Some("d".to_string()),

                        nodes: vec![TreeNode {
                            name: Some("e".to_string()),

                            nodes: vec![TreeNode {
                                name: Some("f".to_string()),
                                focused: true,
                                ..Default::default()
                            }],
                            ..Default::default()
                        }],
                        floating_nodes: vec![TreeNode {
                            name: Some("g".to_string()),

                            nodes: vec![TreeNode {
                                name: Some("h".to_string()),
                                focused: true,
                                ..Default::default()
                            }],
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    TreeNode {
                        name: Some("i".to_string()),

                        nodes: vec![TreeNode {
                            name: Some("j".to_string()),
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
                &res.unwrap().node.name.clone().unwrap_or("".to_string())
            );
        }

        #[test]
        fn navigation_next_sibling() {
            let tree = build_tree();
            let res: Result<Cursor, Cursor> =
                (|| Cursor::new(&tree).descend()?.descend()?.next_sibling())();
            assert_eq!(
                "d",
                &res.unwrap().node.name.clone().unwrap_or("".to_string())
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
                &res.unwrap().node.name.clone().unwrap_or("".to_string())
            );
        }

        #[test]
        fn is_floating() {
            let tree = build_tree();
            let res: Result<Cursor, Cursor> = (|| {
                Cursor::new(&tree)
                    .descend()?
                    .descend()?
                    .next_sibling()?
                    .descend()
            })();
            assert_eq!(
                "e",
                &res.clone()
                    .unwrap()
                    .node
                    .name
                    .clone()
                    .unwrap_or("".to_string())
            );
            assert_eq!(false, res.unwrap().is_floating());
        }

        #[test]
        fn is_not_floating() {
            let tree = build_tree();
            let res: Result<Cursor, Cursor> = (|| {
                Cursor::new(&tree)
                    .descend()?
                    .descend()?
                    .next_sibling()?
                    .descend()?
                    .next_sibling()
            })();
            assert_eq!(
                "g",
                &res.clone()
                    .unwrap()
                    .node
                    .name
                    .clone()
                    .unwrap_or("".to_string())
            );
            assert_eq!(true, res.unwrap().is_floating());
        }
    }

    mod iterator {
        use super::*;
        #[test]
        fn traversal() {
            let tree = build_tree();

            let names = CursorIterator::new(Cursor::new(&tree))
                .map(|c| c.node.name.clone().unwrap_or("".to_string()))
                .collect::<Vec<String>>();

            assert_eq!(names.join(""), "cfehgdjiba".to_string());
        }

        #[test]
        fn into_iter() {
            let tree = build_tree();
            let do_iter = |i: CursorIterator<'_>| {
                i.map(|c| c.node.name.clone().unwrap_or("".to_string()))
                    .collect::<Vec<String>>()
            };

            assert_eq!(
                do_iter(CursorIterator::new(Cursor::new(&tree))),
                do_iter(tree.into_iter())
            );
        }

        #[test]
        fn find() {
            let tree = build_tree();
            let mut co = CursorIterator::new(Cursor::new(&tree)).find(|c| c.node.focused);
            let mut trace: String = Default::default();
            while let Some(c) = co {
                trace += &c.node.name.clone().unwrap_or("".to_string());
                co = c.ascend().ok();
            }
            assert_eq!(&trace, "fedba");
        }
    }
}
