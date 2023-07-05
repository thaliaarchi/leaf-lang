use std::fmt;

use crate::{MultiTree, Node, NodeId};

#[derive(Clone, Debug)]
pub struct TreeCursor {
    tree: MultiTree,
    cursor: NodeId,
    root_stack: Vec<NodeId>,
}

impl TreeCursor {
    pub fn new() -> Self {
        let mut tree = MultiTree::new();
        let root = tree.new_node();
        TreeCursor {
            tree,
            cursor: root,
            root_stack: vec![root],
        }
    }

    pub fn move_left(&mut self) -> bool {
        if let Some(left) = self.node().left() {
            self.cursor = left;
            true
        } else {
            false
        }
    }

    pub fn move_right(&mut self) -> bool {
        if let Some(right) = self.node().right() {
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

    pub fn new_left(&mut self) {
        self.tree.new_left(self.cursor);
    }

    pub fn new_right(&mut self) {
        self.tree.new_right(self.cursor);
    }

    pub fn delete(&mut self) -> bool {
        if let Some(parent) = self.parent() {
            self.tree.delete(self.cursor);
            self.cursor = parent;
            true
        } else {
            false
        }
    }

    pub fn unrooted(&self) -> &MultiTree {
        &self.tree
    }

    pub fn into_unrooted(self) -> MultiTree {
        self.tree
    }

    pub fn cursor(&self) -> NodeId {
        self.cursor
    }

    fn node(&self) -> &Node {
        self.tree.get_unchecked(self.cursor)
    }

    fn parent(&self) -> Option<NodeId> {
        if !self.at_root() {
            let parent = self.node().parent();
            debug_assert!(parent.is_some());
            parent
        } else {
            None
        }
    }

    pub fn push_root(&mut self) {
        self.root_stack.push(self.cursor);
    }

    pub fn pop_root(&mut self) -> Option<NodeId> {
        if self.root_stack.len() >= 1 {
            self.root_stack.pop()
        } else {
            None
        }
    }

    pub fn root_stack(&self) -> &[NodeId] {
        &self.root_stack
    }

    pub fn at_root(&self) -> bool {
        self.cursor == self.root_stack[self.root_stack.len() - 1]
    }

    pub fn dump_dot<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        self.tree.dump_dot(w, self.root_stack[0])
    }

    pub fn dump_dot_to_string(&self) -> String {
        self.tree.dump_dot_to_string(self.root_stack[0])
    }
}
