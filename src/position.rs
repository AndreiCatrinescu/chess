use crate::{board::board_columns, piece::PieceType};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl Position {
    pub fn new(row: usize, column: usize) -> Self {
        Position { row, column }
    }
}
pub struct NotationError;

pub struct Move {
    pub piece_type: PieceType,
    pub new_position: Position,
    pub starting_column: Option<usize>,
    pub starting_row: Option<usize>,
}

impl Move {
    pub fn new(
        new_position: Position,
        starting_column: Option<usize>,
        starting_row: Option<usize>,
        piece_type: PieceType,
    ) -> Self {
        Move {
            piece_type,
            new_position,
            starting_column,
            starting_row,
        }
    }

    pub fn from_notation(notation: &str) -> Result<Self, NotationError> {
        let notation_elements: Vec<char> = notation.chars().collect();

        if notation_elements.len() < 2 {
            return Err(NotationError);
        }

        if notation_elements.len() > 6 {
            return Err(NotationError);
        }

        let last: usize = notation_elements.len() - 1;
        let row: usize = match notation_elements[last].to_digit(10) {
            Some(row) => row as usize,
            None => return Err(NotationError),
        };
        let column: usize = match notation_elements[last - 1].to_ascii_uppercase() {
            'A' => board_columns::A,
            'B' => board_columns::B,
            'C' => board_columns::C,
            'D' => board_columns::D,
            'E' => board_columns::E,
            'F' => board_columns::F,
            'G' => board_columns::G,
            'H' => board_columns::H,
            _ => return Err(NotationError),
        };
        let end_position: Position = Position::new(row, column);

        if notation_elements.len() == 2 {
            return Ok(Move::new(end_position, None, None, PieceType::Pawn));
        }

        let piece_type: PieceType = match notation_elements[0] {
            'R' => PieceType::Rook,
            'B' => PieceType::Bishop,
            'K' => PieceType::King,
            'N' => PieceType::Knight,
            'Q' => PieceType::Queen,
            'a'..'h' => PieceType::Pawn,
            _ => return Err(NotationError),
        };

        if notation_elements.len() == 3 {
            if piece_type != PieceType::Pawn {
                return Ok(Move::new(end_position, None, None, piece_type));
            } else {
                let starting_column = match notation_elements[0] {
                    'a' => Some(board_columns::A),
                    'b' => Some(board_columns::B),
                    'c' => Some(board_columns::C),
                    'd' => Some(board_columns::D),
                    'e' => Some(board_columns::E),
                    'f' => Some(board_columns::F),
                    'g' => Some(board_columns::G),
                    'h' => Some(board_columns::H),
                    _ => return Err(NotationError),
                };
                return Ok(Move::new(end_position, starting_column, None, piece_type));
            }
        }

        if notation_elements.len() == 4 {
            if notation_elements[1].is_digit(10) {
                let starting_row: Option<usize> = match notation_elements[1].to_digit(10) {
                    Some(row) => Some(row as usize),
                    None => return Err(NotationError),
                };
                return Ok(Move::new(end_position, None, starting_row, piece_type));
            } else {
                let starting_column: Option<usize> = match notation_elements[1].to_ascii_uppercase()
                {
                    'A' => Some(board_columns::A),
                    'B' => Some(board_columns::B),
                    'C' => Some(board_columns::C),
                    'D' => Some(board_columns::D),
                    'E' => Some(board_columns::E),
                    'F' => Some(board_columns::F),
                    'G' => Some(board_columns::G),
                    'H' => Some(board_columns::H),
                    _ => return Err(NotationError),
                };
                return Ok(Move::new(end_position, starting_column, None, piece_type));
            };
        }

        let starting_column: Option<usize> = match notation_elements[1].to_ascii_uppercase() {
            'A' => Some(board_columns::A),
            'B' => Some(board_columns::B),
            'C' => Some(board_columns::C),
            'D' => Some(board_columns::D),
            'E' => Some(board_columns::E),
            'F' => Some(board_columns::F),
            'G' => Some(board_columns::G),
            'H' => Some(board_columns::H),
            _ => return Err(NotationError),
        };

        let starting_row: Option<usize> = match notation_elements[2].to_digit(10) {
            Some(row) => Some(row as usize),
            None => return Err(NotationError),
        };

        Ok(Move::new(
            end_position,
            starting_column,
            starting_row,
            piece_type,
        ))
    }
}

pub enum MoveResult {
    Success,
    PromotionAvailable(Position),
    ImpossibleMove,
    Checked,
    PiecePinned,
    AmbiguousMove,
    MissingPiece,
    Draw,
}
