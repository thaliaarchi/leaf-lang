use thiserror::Error;

use crate::tree::RootedTree;
use crate::{Inst, Program};

#[derive(Clone, Debug)]
pub struct VM {
    pub(crate) prog: Program,
    pub(crate) pc: usize,
    pub(crate) tree: RootedTree,
    pub(crate) loop_stack: Vec<(usize, usize)>,
    pub(crate) success: bool,
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum VMError {
    #[error("VM has terminated")]
    Terminated,
}

impl VM {
    pub fn new(prog: Program) -> Self {
        VM {
            prog,
            pc: 0,
            tree: RootedTree::new(),
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
                    self.success = self.tree.move_left();
                }
                Inst::MoveRight => {
                    self.success = self.tree.move_right();
                }
                Inst::MoveUp => {
                    self.success = self.tree.move_up();
                }
                Inst::PushRoot => {
                    self.tree.push_root();
                }
                Inst::PopRoot => {
                    self.success = self.tree.pop_root().is_some();
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
                    self.tree.new_left();
                    self.success = true;
                }
                Inst::NewRight => {
                    self.tree.new_right();
                    self.success = true;
                }
                Inst::Delete => {
                    self.success = self.tree.delete();
                }
                Inst::Break => {
                    self.success = self.tree.at_root();
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

    pub fn program(&self) -> &Program {
        &self.prog
    }

    pub fn tree(&self) -> &RootedTree {
        &self.tree
    }
}
