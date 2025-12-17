use crate::Direction;
use crate::Symbol;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TapeError {
    LeftOfStart,
}

impl fmt::Display for TapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TapeError::LeftOfStart => write!(f, "Error: Head cannot move left."),
        }
    }
}

impl Error for TapeError {}

#[derive(Debug)]
pub struct Tape {
    contents: Vec<Symbol>,
    head: usize,
}

impl Tape {
    pub fn new(input: Vec<Symbol>) -> Self {
        let contents = if input.is_empty() {
            vec![Symbol::Blank]
        } else {
            input
        };
        Tape { contents, head: 0 }
    }

    pub fn read(&self) -> Symbol {
        self.contents[self.head].clone()
    }

    pub fn write(&mut self, symbol: Symbol) {
        self.contents[self.head] = symbol;
    }

    pub fn move_head(&mut self, direction: Direction) -> Result<(), TapeError> {
        match direction {
            Direction::Right => {
                self.head += 1;
                if self.head >= self.contents.len() {
                    self.contents.push(Symbol::Blank);
                }
                Ok(())
            }
            Direction::Left => {
                if self.head == 0 {
                    Err(TapeError::LeftOfStart)
                } else {
                    self.head -= 1;
                    Ok(())
                }
            }
        }
    }
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, symbol) in self.contents.iter().enumerate() {
            let char = symbol.to_char();
            if i == self.head {
                write!(f, "[{}] ", char)?;
            } else {
                write!(f, "{} ", char)?;
            }
        }
        Ok(())
    }
}
