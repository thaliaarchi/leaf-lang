use crate::{Inst, Program};

#[derive(Clone, Debug)]
pub struct VM<'a> {
    prog: &'a Program,
    pc: usize,
}

impl<'a> VM<'a> {
    pub fn new(prog: &'a Program) -> Self {
        VM { prog, pc: 0 }
    }

    pub fn step(&mut self) {
        if let Some(inst) = self.prog.get(self.pc) {
            match inst {
                Inst::MoveLeft => todo!(),
                Inst::MoveRight => todo!(),
                Inst::MoveUp => todo!(),
                Inst::PushRoot => todo!(),
                Inst::PopRoot => todo!(),
                Inst::LoopHead => todo!(),
                Inst::LoopTail(_head) => todo!(),
                Inst::AddLeft => todo!(),
                Inst::AddRight => todo!(),
                Inst::Delete => todo!(),
                Inst::BreakRoot => todo!(),
            }
        }
    }
}
