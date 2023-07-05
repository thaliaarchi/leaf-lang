use thiserror::Error;

use crate::{Inst, Program, TreeCursor};

#[derive(Clone, Debug)]
pub struct VM<'a> {
    prog: &'a Program,
    pc: usize,
    tree: TreeCursor,
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
        VM {
            prog,
            pc: 0,
            tree: TreeCursor::new(),
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
                    self.tree.pop_root().ok_or(VMError::PopRootEmpty)?;
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

    pub fn tree(&self) -> &TreeCursor {
        &self.tree
    }
}
