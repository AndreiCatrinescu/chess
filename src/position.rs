#[derive(PartialEq, Eq, Clone, Copy)]
#[allow(dead_code)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl Position {
    pub fn new(row: usize, column: usize) -> Position {
        Position { row, column }
    }
}

pub struct Move {
    pub end_position: Position,
    pub starting_column: Option<usize>,
    pub starting_row: Option<usize>,
}
