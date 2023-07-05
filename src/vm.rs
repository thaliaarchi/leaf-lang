use thiserror::Error;

use crate::{Inst, Program, Tree, TreeRef};

#[derive(Clone, Debug)]
pub struct VM<'a> {
    prog: &'a Program,
    pc: usize,
    tree: Tree,
    root: TreeRef,
    cursor: TreeRef,
    root_stack: Vec<TreeRef>,
    loop_stack: Vec<(usize, usize)>,
    success: bool,
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum VMError {
    #[error("cannot pop root (`}}` without `{{`)")]
    PopRootEmpty,
    #[error("VM has terminated")]
    Terminated,
}

impl<'a> VM<'a> {
    pub fn new(prog: &'a Program) -> Self {
        let mut tree = Tree::new();
        let root = tree.new_empty();
        VM {
            prog,
            pc: 0,
            tree,
            root,
            cursor: root,
            root_stack: vec![root],
            loop_stack: vec![],
            success: false,
        }
    }

    pub fn run(&mut self) -> Result<(), VMError> {
        if self.pc >= self.prog.len() {
            return Err(VMError::Terminated);
        }
        loop {
            if let Err(err) = self.step_inline() {
                if err == VMError::Terminated {
                    return Ok(());
                } else {
                    return Err(err);
                }
            }
        }
    }

    pub fn step(&mut self) -> Result<(), VMError> {
        self.step_inline()
    }

    #[inline(always)]
    fn step_inline(&mut self) -> Result<(), VMError> {
        if let Some(inst) = self.prog.get(self.pc) {
            match *inst {
                Inst::MoveLeft => {
                    self.success = false;
                    if let Some(left) = self.tree.get(self.cursor).left() {
                        self.cursor = left;
                        self.success = true;
                    }
                }
                Inst::MoveRight => {
                    self.success = false;
                    if let Some(right) = self.tree.get(self.cursor).right() {
                        self.cursor = right;
                        self.success = true;
                    }
                }
                Inst::MoveUp => {
                    self.success = false;
                    if let Some(parent) = self.tree.get(self.cursor).parent() {
                        if self.cursor != self.current_root() {
                            self.cursor = parent;
                            self.success = true;
                        }
                    }
                }
                Inst::PushRoot => {
                    self.root_stack.push(self.cursor);
                }
                Inst::PopRoot => {
                    if self.root_stack.len() <= 1 {
                        return Err(VMError::PopRootEmpty);
                    }
                    self.root_stack.pop();
                    self.success = true;
                }
                Inst::LoopHead(tail) => {
                    self.loop_stack.push((self.pc, tail));
                }
                Inst::LoopTail => {
                    if self.success {
                        let (head, _) = self.loop_stack[self.loop_stack.len() - 1];
                        self.pc = head;
                    } else {
                        self.loop_stack.pop().unwrap();
                    }
                    self.success = true;
                }
                Inst::NewLeft => {
                    self.tree.set_left_empty(self.cursor);
                    self.success = true;
                }
                Inst::NewRight => {
                    self.tree.set_right_empty(self.cursor);
                    self.success = true;
                }
                Inst::Delete => {
                    let deleted = self.cursor;
                    self.success = false;
                    if let Some(parent) = self.tree.get(self.cursor).parent() {
                        self.cursor = parent;
                        self.success = true;
                    }
                    self.tree.remove(deleted);
                }
                Inst::Break => {
                    self.success = self.cursor == self.current_root();
                    if self.success {
                        self.pc = if let Some((_, tail)) = self.loop_stack.pop() {
                            tail
                        } else {
                            self.prog.len()
                        };
                    }
                }
            }
            self.pc += 1;
            Ok(())
        } else {
            Err(VMError::Terminated)
        }
    }

    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    pub fn root(&self) -> TreeRef {
        self.root
    }

    pub fn current_root(&self) -> TreeRef {
        self.root_stack[self.root_stack.len() - 1]
    }
}
