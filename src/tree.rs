use std::collections::vec_deque::VecDeque;
use std::fmt;
use std::num::NonZeroUsize;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct MultiTree {
    nodes: Vec<Node>,
    free: Option<NodeId>,
}

#[derive(Clone, Debug)]
pub struct Node {
    left: Option<NodeId>,
    right: Option<NodeId>,
    parent: Option<NodeId>,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeId(NonZeroUsize);

impl MultiTree {
    pub fn new() -> Self {
        MultiTree {
            nodes: Vec::new(),
            free: None,
        }
    }

    pub fn new_node(&mut self) -> NodeId {
        if let Some(id) = self.free {
            let node = self.get_unchecked_mut(id);
            let left = node.left;
            let right = node.right;
            let next_free = node.parent;
            node.left = None;
            node.right = None;
            node.parent = None;
            self.free = next_free;
            if let Some(right) = right {
                self.free(right);
            }
            if let Some(left) = left {
                self.free(left);
            }
            id
        } else {
            let id = NodeId::new(self.nodes.len());
            self.nodes.push(Node {
                left: None,
                right: None,
                parent: None,
            });
            id
        }
    }

    pub fn new_left(&mut self, id: NodeId) {
        let left = self.new_node_reused(self[id].left, Some(id));
        self.get_unchecked_mut(id).left = Some(left);
    }

    pub fn new_right(&mut self, id: NodeId) {
        let right = self.new_node_reused(self[id].right, Some(id));
        self.get_unchecked_mut(id).right = Some(right);
    }

    fn new_node_reused(&mut self, id: Option<NodeId>, parent: Option<NodeId>) -> NodeId {
        if let Some(id) = id {
            self.free(id);
        }
        let id = self.new_node();
        self[id].parent = parent;
        id
    }

    pub fn set_left(&mut self, id: NodeId, left: Option<NodeId>) {
        let node = &mut self[id];
        let old = node.left;
        node.left = left;
        if let Some(left) = left {
            self.get_unchecked_mut(left).parent = Some(id);
        }
        if let Some(old_left) = old {
            self.free(old_left);
        }
    }

    pub fn set_right(&mut self, id: NodeId, right: Option<NodeId>) {
        let node = &mut self[id];
        let old = node.right;
        node.right = right;
        if let Some(right) = right {
            self.get_unchecked_mut(right).parent = Some(id);
        }
        if let Some(old_right) = old {
            self.free(old_right);
        }
    }

    pub fn delete(&mut self, id: NodeId) {
        let parent = self[id].parent;
        self.free(id);
        if let Some(parent) = parent {
            let parent = self.get_unchecked_mut(parent);
            if parent.left == Some(id) {
                parent.left = None;
            } else if parent.right == Some(id) {
                parent.right = None;
            }
        }
    }

    fn free(&mut self, id: NodeId) {
        self[id].parent = self.free;
        self.free = Some(id);
    }

    pub(crate) fn get_unchecked(&self, id: NodeId) -> &Node {
        unsafe { self.nodes.get_unchecked(id.as_usize()) }
    }

    pub(crate) fn get_unchecked_mut(&mut self, id: NodeId) -> &mut Node {
        unsafe { self.nodes.get_unchecked_mut(id.as_usize()) }
    }

    pub fn dump_dot<W: fmt::Write>(&self, w: &mut W, id: NodeId) -> fmt::Result {
        _ = self[id]; // Bounds check
        writeln!(w, "digraph tree{} {{", id.0)?;
        let mut stack = VecDeque::new();
        stack.push_back(id);
        while let Some(id) = stack.pop_front() {
            writeln!(w, "    {} [shape=point];", id.0)?;
            let node = self.get_unchecked(id);
            if let Some(left) = node.left {
                writeln!(w, "    {} -> {};", id.0, left.0)?;
                stack.push_back(left);
            }
            if let Some(right) = node.right {
                writeln!(w, "    {} -> {} [style=dashed];", id.0, right.0)?;
                stack.push_back(right);
            }
        }
        writeln!(w, "}}")
    }

    pub fn dump_dot_to_string(&self, id: NodeId) -> String {
        let mut s = String::new();
        self.dump_dot(&mut s, id).unwrap();
        s
    }
}

impl Index<NodeId> for MultiTree {
    type Output = Node;

    fn index(&self, id: NodeId) -> &Self::Output {
        &self.nodes[id.as_usize()]
    }
}

impl IndexMut<NodeId> for MultiTree {
    fn index_mut(&mut self, id: NodeId) -> &mut Self::Output {
        &mut self.nodes[id.as_usize()]
    }
}

impl Node {
    pub fn left(&self) -> Option<NodeId> {
        self.left
    }

    pub fn right(&self) -> Option<NodeId> {
        self.right
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }
}

impl NodeId {
    fn new(id: usize) -> Self {
        NodeId(unsafe { NonZeroUsize::new_unchecked(id + 1) })
    }

    fn as_usize(self) -> usize {
        self.0.get() - 1
    }
}
