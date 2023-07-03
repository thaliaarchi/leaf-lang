#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Inst {
    /// `<`
    MoveLeft,
    /// `>`
    MoveRight,
    /// `^`
    MoveUp,
    /// `{`
    PushRoot,
    /// `}`
    PopRoot,
    /// `(`
    LoopHead(usize),
    /// `)`
    LoopTail,
    /// `+`
    AddLeft,
    /// `*`
    AddRight,
    /// `-`
    Delete,
    /// `?`
    BreakRoot,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Program(Vec<Inst>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ParseError {
    UnopenedLoop,
    UnclosedLoop,
}

impl Program {
    pub fn parse(src: &str) -> Result<Program, ParseError> {
        let mut prog = Vec::new();
        let mut loops = Vec::new();
        for ch in src.as_bytes() {
            let inst = match ch {
                b'<' => Inst::MoveLeft,
                b'>' => Inst::MoveRight,
                b'^' => Inst::MoveUp,
                b'{' => Inst::PushRoot,
                b'}' => Inst::PopRoot,
                b'(' => {
                    loops.push(prog.len());
                    Inst::LoopHead(usize::MAX)
                }
                b')' => {
                    let head = loops.pop().ok_or(ParseError::UnopenedLoop)?;
                    prog[head] = Inst::LoopHead(prog.len());
                    Inst::LoopTail
                }
                b'+' => Inst::AddLeft,
                b'*' => Inst::AddRight,
                b'-' => Inst::Delete,
                b'?' => Inst::BreakRoot,
                _ => continue,
            };
            prog.push(inst);
        }
        if !loops.is_empty() {
            return Err(ParseError::UnclosedLoop);
        }
        Ok(Program(prog))
    }

    pub fn get(&self, pc: usize) -> Option<&Inst> {
        self.0.get(pc)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
