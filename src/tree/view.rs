use std::fmt;

use crate::tree::{MultiTree, Node, NodeId};

#[derive(Clone, Debug)]
pub struct TreeView<'a> {
    tree: &'a MultiTree,
    cursor: NodeId,
}

impl<'a> TreeView<'a> {
    pub fn new(tree: &'a MultiTree, cursor: NodeId) -> Self {
        TreeView { tree, cursor }
    }

    pub fn node(&self) -> &Node {
        self.tree.get_unchecked(self.cursor)
    }

    pub fn left(&self) -> Option<NodeId> {
        self.node().left()
    }

    pub fn right(&self) -> Option<NodeId> {
        self.node().right()
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.node().parent()
    }

    pub fn move_left(&mut self) -> bool {
        if let Some(left) = self.left() {
            self.cursor = left;
            true
        } else {
            false
        }
    }

    pub fn move_right(&mut self) -> bool {
        if let Some(right) = self.right() {
            self.cursor = right;
            true
        } else {
            false
        }
    }

    pub fn move_up(&mut self) -> bool {
        if let Some(parent) = self.parent() {
            self.cursor = parent;
            true
        } else {
            false
        }
    }

    pub fn at(&self, cursor: NodeId) -> TreeView {
        TreeView::new(self.tree, cursor)
    }

    pub fn count_left(mut self) -> Option<usize> {
        let mut n = 0;
        while self.move_left() {
            n += 1;
        }
        Some(n)
    }

    pub fn count_right(mut self) -> Option<usize> {
        let mut n = 0;
        while self.move_right() {
            n += 1;
        }
        Some(n)
    }

    pub fn count_left_only(mut self) -> Option<usize> {
        let mut n = 0;
        loop {
            if self.right().is_some() {
                return None;
            }
            if self.move_left() {
                n += 1;
            } else {
                return Some(n);
            }
        }
    }

    pub fn count_right_only(mut self) -> Option<usize> {
        let mut n = 0;
        loop {
            if self.left().is_some() {
                return None;
            }
            if self.move_right() {
                n += 1;
            } else {
                return Some(n);
            }
        }
    }

    pub fn dump_dot<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        self.tree.dump_dot(w, self.cursor)
    }

    pub fn dump_dot_to_string(&self) -> String {
        self.tree.dump_dot_to_string(self.cursor)
    }
}
