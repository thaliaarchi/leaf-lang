use thiserror::Error;

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
    NewLeft,
    /// `*`
    NewRight,
    /// `-`
    Delete,
    /// `?`
    Break,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Program(Vec<Inst>);

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("unopened loop (`)` without `(`)")]
    UnopenedLoop,
    #[error("unclosed loop (`(` without `)`)")]
    UnclosedLoop,
}

impl Program {
    pub fn parse(src: &str) -> Result<Self, ParseError> {
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
                b'+' => Inst::NewLeft,
                b'*' => Inst::NewRight,
                b'-' => Inst::Delete,
                b'?' => Inst::Break,
                _ => continue,
            };
            prog.push(inst);
        }
        if !loops.is_empty() {
            return Err(ParseError::UnclosedLoop);
        }
        Ok(Program(prog))
    }

    pub fn from_insts(mut prog: Vec<Inst>) -> Result<Self, ParseError> {
        let mut loops = Vec::new();
        for pc in 0..prog.len() {
            match prog[pc] {
                Inst::LoopHead(_) => loops.push(pc),
                Inst::LoopTail => {
                    let head = loops.pop().ok_or(ParseError::UnopenedLoop)?;
                    prog[head] = Inst::LoopHead(pc);
                }
                _ => {}
            }
        }
        if !loops.is_empty() {
            return Err(ParseError::UnclosedLoop);
        }
        Ok(Program(prog))
    }

    pub fn insts(&self) -> &[Inst] {
        &self.0
    }

    pub fn get(&self, pc: usize) -> Option<&Inst> {
        self.0.get(pc)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Into<Vec<Inst>> for Program {
    fn into(self) -> Vec<Inst> {
        self.0
    }
}
