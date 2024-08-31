#[derive(PartialEq, Eq, Clone, Copy, Hash)]
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

impl Move {
    pub fn new(
        end_position: Position,
        starting_column: Option<usize>,
        starting_row: Option<usize>,
    ) -> Self {
        Move {
            end_position,
            starting_column,
            starting_row,
        }
    }
}
