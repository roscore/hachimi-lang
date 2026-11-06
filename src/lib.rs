//! Parser and bounded tape VM for Hachimi Lang.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Increment,
    Decrement,
    Right,
    Left,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub op: Op,
    pub position: Position,
    pub jump: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    UnmatchedLoopStart,
    UnmatchedLoopEnd,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub position: Position,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self.kind {
            ParseErrorKind::UnmatchedLoopStart => "unmatched 맘보 (loop start)",
            ParseErrorKind::UnmatchedLoopEnd => "unmatched 띵띵똥똥 (loop end)",
        };
        write!(
            f,
            "{}:{}: {message}",
            self.position.line, self.position.column
        )
    }
}

impl std::error::Error for ParseError {}

pub fn parse(source: &str) -> Result<Program, ParseError> {
    const TOKENS: [(&str, Op); 8] = [
        ("하찌미", Op::Increment),
        ("아시가", Op::Decrement),
        ("하야쿠", Op::Right),
        ("나루", Op::Left),
        ("테이오", Op::Output),
        ("허니", Op::Input),
        ("맘보", Op::LoopStart),
        ("띵띵똥똥", Op::LoopEnd),
    ];

    let mut instructions: Vec<Instruction> = Vec::new();
    let mut loop_starts = Vec::new();
    let (mut offset, mut line, mut column) = (0, 1, 1);
    let mut previous_was_cr = false;

    while offset < source.len() {
        let remainder = &source[offset..];
        if let Some((token, op)) = TOKENS
            .iter()
            .find(|(token, _)| remainder.starts_with(token))
        {
            let index = instructions.len();
            let position = Position { line, column };
            instructions.push(Instruction {
                op: *op,
                position,
                jump: None,
            });
            match op {
                Op::LoopStart => loop_starts.push(index),
                Op::LoopEnd => {
                    let start = loop_starts.pop().ok_or(ParseError {
                        kind: ParseErrorKind::UnmatchedLoopEnd,
                        position,
                    })?;
                    instructions[start].jump = Some(index);
                    instructions[index].jump = Some(start);
                }
                _ => {}
            }
            offset += token.len();
            column += token.chars().count();
            previous_was_cr = false;
        } else {
            let ch = remainder.chars().next().expect("nonempty remainder");
            offset += ch.len_utf8();
            if ch == '\r' {
                line += 1;
                column = 1;
                previous_was_cr = true;
            } else if ch == '\n' {
                if !previous_was_cr {
                    line += 1;
                }
                column = 1;
                previous_was_cr = false;
            } else {
                column += 1;
                previous_was_cr = false;
            }
        }
    }

    if let Some(start) = loop_starts.pop() {
        return Err(ParseError {
            kind: ParseErrorKind::UnmatchedLoopStart,
            position: instructions[start].position,
        });
    }

    Ok(Program { instructions })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    pub max_steps: usize,
    pub max_cells: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_steps: 1_000_000,
            max_cells: 30_000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeErrorKind {
    PointerUnderflow,
    StepLimitExceeded,
    CellLimitExceeded,
    InvalidCellLimit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    pub position: Option<Position>,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self.kind {
            RuntimeErrorKind::PointerUnderflow => "pointer moved left of cell 0",
            RuntimeErrorKind::StepLimitExceeded => "step limit exceeded",
            RuntimeErrorKind::CellLimitExceeded => "cell limit exceeded",
            RuntimeErrorKind::InvalidCellLimit => "max_cells must be at least 1",
        };
        if let Some(position) = self.position {
            write!(f, "{}:{}: {message}", position.line, position.column)
        } else {
            f.write_str(message)
        }
    }
}

impl std::error::Error for RuntimeError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Execution {
    pub output: Vec<u8>,
    pub tape: Vec<u8>,
    pub pointer: usize,
    pub steps: usize,
}

pub fn execute(program: &Program, input: &[u8], limits: Limits) -> Result<Execution, RuntimeError> {
    if limits.max_cells == 0 {
        return Err(RuntimeError {
            kind: RuntimeErrorKind::InvalidCellLimit,
            position: None,
        });
    }

    let mut result = Execution {
        output: Vec::new(),
        tape: vec![0],
        pointer: 0,
        steps: 0,
    };
    let (mut pc, mut input_index) = (0, 0);
    while let Some(instruction) = program.instructions.get(pc) {
        let error = |kind| RuntimeError {
            kind,
            position: Some(instruction.position),
        };
        if result.steps >= limits.max_steps {
            return Err(error(RuntimeErrorKind::StepLimitExceeded));
        }
        result.steps += 1;
        match instruction.op {
            Op::Increment => {
                result.tape[result.pointer] = result.tape[result.pointer].wrapping_add(1);
            }
            Op::Decrement => {
                result.tape[result.pointer] = result.tape[result.pointer].wrapping_sub(1);
            }
            Op::Right => {
                if result.pointer + 1 >= limits.max_cells {
                    return Err(error(RuntimeErrorKind::CellLimitExceeded));
                }
                result.pointer += 1;
                if result.pointer == result.tape.len() {
                    result.tape.push(0);
                }
            }
            Op::Left => {
                if result.pointer == 0 {
                    return Err(error(RuntimeErrorKind::PointerUnderflow));
                }
                result.pointer -= 1;
            }
            Op::Output => result.output.push(result.tape[result.pointer]),
            Op::Input => {
                result.tape[result.pointer] = input.get(input_index).copied().unwrap_or(0);
                input_index += 1;
            }
            Op::LoopStart if result.tape[result.pointer] == 0 => {
                pc = instruction.jump.expect("parsed loop has a match") + 1;
                continue;
            }
            Op::LoopEnd if result.tape[result.pointer] != 0 => {
                pc = instruction.jump.expect("parsed loop has a match") + 1;
                continue;
            }
            Op::LoopStart | Op::LoopEnd => {}
        }
        pc += 1;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{parse, ParseErrorKind, Position};

    #[test]
    fn bare_cr_and_crlf_each_count_as_one_line_break() {
        let error = parse("한글\r메모\r\n 맘보").unwrap_err();
        assert_eq!(error.kind, ParseErrorKind::UnmatchedLoopStart);
        assert_eq!(error.position, Position { line: 3, column: 2 });
    }
}
