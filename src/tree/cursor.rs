//! Utilities for easily traversing a sway layout tree keeping track of where one is in the tree.

use std::{default::Default, rc::Rc};

use crate::tree::TreeNode;

/// Find the currently focused node in the sway tree layout.
pub fn find_focused(root: &TreeNode) -> Option<Cursor> {
    root.into_iter().find(|c| c.node.focused)
}

#[derive(Debug, Clone)]
/// A cursor into the sway tree layout which keeps track of where it is in the tree.
///
/// This also abstracts over floating and tiling children of nodes when navigating. Floating
/// children appear after tiling children.
pub struct Cursor<'a> {
    parent: Option<Rc<Cursor<'a>>>,
    node: &'a TreeNode,
    idx_in_parent: usize,
}

impl<'a> Cursor<'a> {
    /// Create a new cursor given a root tree node.
    pub fn new(node: &'a TreeNode) -> Cursor<'a> {
        Cursor {
            node,
            parent: Default::default(),
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

    /// Get the node associated with the cursor
    pub fn get_node(&self) -> &'a TreeNode {
        self.node
    }

    /// Is the node associated with the cursor a floating node
    pub fn is_floating(&self) -> bool {
        match &self.parent {
            None => false,
            Some(parent) => self.idx_in_parent >= parent.node.nodes.len(),
        }
    }

    /// Get the ancestors from the current node under focus with the immediate parent being the
    /// first element.
    pub fn ancestors(&self) -> Vec<Self> {
        let mut vec: Vec<Self> = Default::default();
        let mut next = &self.parent;
        while let Some(parent) = &next {
            vec.push(parent.as_ref().clone());
            next = &parent.parent;
        }
        vec
    }

    /// Descend into the first child node if possible or return self on failure.
    pub fn descend(mut self) -> Result<Self, Self> {
        match self.deref_child(0) {
            None => Err(self),
            Some(child) => {
                self.parent = Some(Rc::new(self.clone()));
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

    /// Navigate to the previous sibling if there is one or return self on failure.
    pub fn prev_sibling(mut self) -> Result<Self, Self> {
        let parent = match &self.parent {
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

    /// Navigate to the next sibling if there is one or return self on failure.
    pub fn next_sibling(mut self) -> Result<Self, Self> {
        let parent = match &self.parent {
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

    /// Navigate to the parent if there is one or return self on failure.
    pub fn ascend(self) -> Result<Self, Self> {
        match &self.parent {
            None => Result::Err(self),
            Some(x) => Result::Ok(x.as_ref().clone()),
        }
    }

    /// Return an iterator over the tree iterating depth first left to right.
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
/// A depth first left to right iterator over a sway tree hierarchy based on [Cursor]s.
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

        #[test]
        fn ancestors() {
            let tree = build_tree();
            let names = Cursor::new(&tree)
                .into_iter()
                .find(|c| c.node.name == Some("f".to_string()))
                .unwrap()
                .ancestors()
                .into_iter()
                .map(|c| c.node.name.clone().unwrap_or("".to_string()))
                .collect::<Vec<String>>();
            assert_eq!(names.join(""), "edba".to_string());
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
