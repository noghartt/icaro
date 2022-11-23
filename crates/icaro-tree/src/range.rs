#[derive(Debug, Clone, Default)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub index: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Position {
    pub fn new(line: usize, column: usize, index: usize) -> Position {
        Position {
            line,
            column,
            index,
        }
    }

    pub fn advance(&mut self, chr: char) {
        match chr {
            '\n' => {
                self.column = 0;
                self.line += 1;
            }
            _ => {
                self.column += 1;
            }
        }
        self.index += chr.len_utf8();
    }
}

impl Range {
    pub fn new(start: Position, end: Position) -> Range {
        Range { start, end }
    }
}
