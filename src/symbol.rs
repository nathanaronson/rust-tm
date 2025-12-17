#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Symbol {
    Blank,
    Zero,
    One,
}

impl Symbol {
    pub fn to_char(&self) -> char {
        match self {
            Symbol::Blank => '_',
            Symbol::Zero => '0',
            Symbol::One => '1',
        }
    }
}
