#[derive(Debug)]
pub struct Position {
    pub index: u32,
}

#[derive(Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Range {
        Range { start, end  }
    }
}
