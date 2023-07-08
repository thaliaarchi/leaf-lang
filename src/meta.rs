use thiserror::Error;

use crate::{Inst, MultiTree, NodeId, ParseError, Program, TreeCursor, VM};

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

impl TreeCursor {
    pub fn parse_meta_vm(&self) -> Result<VM, ParseMetaVMError> {
        let tree = self.unrooted();
        let mut cursor = self.root_stack()[0];

        let mut prog = Vec::new();
        let mut pc = 0;
        loop {
            let node = &tree[cursor];
            cursor = node.right().ok_or(ParseMetaVMError::MissingTree)?;
            if let Some(left) = node.left() {
                let value = tree
                    .left_int_value(left)
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

        let success = tree[cursor].left().is_some();
        cursor = tree[cursor].right().ok_or(ParseMetaVMError::MissingTree)?;
        if tree[cursor].right().is_some() {
            return Err(ParseMetaVMError::DataAfterTree);
        }
        cursor = tree[cursor].left().ok_or(ParseMetaVMError::MissingTree)?;

        let tree = TreeCursor::new();
        _ = cursor;

        Ok(VM {
            prog,
            pc: usize::MAX,
            tree,
            loop_stack: vec![],
            success,
        })
    }
}

impl MultiTree {
    fn left_int_value(&self, id: NodeId) -> Option<u64> {
        let mut cursor = id;
        let mut n = 0;
        loop {
            let node = &self[cursor];
            if node.right().is_some() {
                return None;
            }
            if let Some(left) = node.left() {
                cursor = left;
                n += 1;
            } else {
                return Some(n);
            }
        }
    }
}
