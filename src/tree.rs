use std::num::NonZeroUsize;

#[derive(Clone, Debug)]
pub struct Tree {
    nodes: Vec<TreeNode>,
    free: Option<TreeRef>,
}

#[derive(Clone, Debug)]
pub struct TreeNode {
    left: Option<TreeRef>,
    right: Option<TreeRef>,
    parent: Option<TreeRef>,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeRef(NonZeroUsize);

impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: Vec::new(),
            free: None,
        }
    }

    pub fn new_empty(&mut self) -> TreeRef {
        if let Some(id) = self.free {
            let node = self.get_mut(id);
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
            let id = TreeRef::new(self.nodes.len());
            self.nodes.push(TreeNode {
                left: None,
                right: None,
                parent: None,
            });
            id
        }
    }

    pub fn set_left(&mut self, id: TreeRef, left: Option<TreeRef>) {
        let node = self.get_mut(id);
        let old = node.left;
        node.left = left;
        if let Some(left) = old {
            self.free(left);
        }
    }

    pub fn set_right(&mut self, id: TreeRef, right: Option<TreeRef>) {
        let node = self.get_mut(id);
        let old = node.right;
        node.right = right;
        if let Some(right) = old {
            self.free(right);
        }
    }

    pub fn set_left_empty(&mut self, id: TreeRef) {
        let left = self.free_or_new(self.get(id).left, Some(id));
        self.get_mut(id).left = Some(left);
    }

    pub fn set_right_empty(&mut self, id: TreeRef) {
        let right = self.free_or_new(self.get(id).right, Some(id));
        self.get_mut(id).right = Some(right);
    }

    fn free_or_new(&mut self, id: Option<TreeRef>, parent: Option<TreeRef>) -> TreeRef {
        if let Some(id) = id {
            self.free(id);
        }
        let id = self.new_empty();
        self.get_mut(id).parent = parent;
        id
    }

    pub fn remove(&mut self, id: TreeRef) {
        let parent = self.get(id).parent;
        self.free(id);
        if let Some(parent) = parent {
            let parent = self.get_mut(parent);
            if parent.left == Some(id) {
                parent.left = None;
            } else if parent.right == Some(id) {
                parent.right = None;
            }
        }
    }

    fn free(&mut self, id: TreeRef) {
        let free = self.free;
        self.get_mut(id).parent = free;
        self.free = Some(id);
    }

    pub fn get(&self, id: TreeRef) -> &TreeNode {
        &self.nodes[id.as_usize()]
    }

    pub fn get_mut(&mut self, id: TreeRef) -> &mut TreeNode {
        &mut self.nodes[id.as_usize()]
    }
}

impl TreeNode {
    pub fn left(&self) -> Option<TreeRef> {
        self.left
    }

    pub fn right(&self) -> Option<TreeRef> {
        self.right
    }

    pub fn parent(&self) -> Option<TreeRef> {
        self.parent
    }
}

impl TreeRef {
    fn new(id: usize) -> Self {
        TreeRef(unsafe { NonZeroUsize::new_unchecked(id + 1) })
    }

    fn as_usize(self) -> usize {
        self.0.get() - 1
    }
}
