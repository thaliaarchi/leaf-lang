use thiserror::Error;

use crate::tree::RootedTree;
use crate::{Inst, ParseError, Program, VM};

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum ParseMetaVMError {
    #[error("invalid number at index {0}")]
    InvalidNumber(usize),
    #[error("invalid opcode at index {0}")]
    InvalidOpcode(usize),
    #[error("program parse: {0}")]
    ProgramError(#[from] ParseError),
    #[error("missing tree")]
    MissingTree,
    #[error("data after tree")]
    DataAfterTree,
}

impl RootedTree {
    pub fn parse_meta_vm(&self) -> Result<VM, ParseMetaVMError> {
        let mut view = self.unrooted().view(self.root_stack()[0]);

        let mut prog = Vec::new();
        let mut pc = 0;
        loop {
            let left = view.left();
            if !view.move_right() {
                return Err(ParseMetaVMError::MissingTree);
            }
            if let Some(left) = left {
                let value = view
                    .at(left)
                    .count_left_only()
                    .ok_or_else(|| ParseMetaVMError::InvalidNumber(pc))?;
                let opcode = match value + 1 {
                    1 => Inst::NewLeft,
                    2 => Inst::NewRight,
                    3 => Inst::MoveLeft,
                    4 => Inst::MoveRight,
                    5 => Inst::MoveUp,
                    6 => Inst::LoopHead(usize::MAX),
                    7 => Inst::LoopTail,
                    8 => Inst::PushRoot,
                    9 => Inst::PopRoot,
                    10 => Inst::Delete,
                    11 => Inst::Break,
                    _ => return Err(ParseMetaVMError::InvalidOpcode(pc)),
                };
                prog.push(opcode);
            } else {
                break;
            }
            pc += 1;
        }
        let prog = Program::from_insts(prog)?;

        let success = view.left().is_some();
        if !view.move_right() {
            return Err(ParseMetaVMError::MissingTree);
        }
        if view.right().is_some() {
            return Err(ParseMetaVMError::DataAfterTree);
        }
        if !view.move_left() {
            return Err(ParseMetaVMError::MissingTree);
        }

        let tree = RootedTree::new();

        Ok(VM {
            prog,
            pc: usize::MAX,
            tree,
            loop_stack: vec![],
            success,
        })
    }
}
