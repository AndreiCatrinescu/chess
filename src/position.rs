use crate::{board::board_columns, piece::PieceType};

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
    pub piece_type: PieceType,
    pub end_position: Position,
    pub starting_column: Option<usize>,
    pub starting_row: Option<usize>,
}

impl Move {
    pub fn new(
        end_position: Position,
        starting_column: Option<usize>,
        starting_row: Option<usize>,
        piece_type: PieceType,
    ) -> Self {
        Move {
            piece_type,
            end_position,
            starting_column,
            starting_row,
        }
    }

    // pub fn from_notation(notation: &str) -> Result<Self, &'static str> {
    //     let chars: Vec<char> = notation.chars().collect();
    //     if chars.len() == 2 {
    //         let column: usize = match chars[0].to_ascii_uppercase() {
    //             'A' => board_columns::A,
    //             'B' => board_columns::B,
    //             'C' => board_columns::C,
    //             'D' => board_columns::D,
    //             'E' => board_columns::E,
    //             'F' => board_columns::F,
    //             'G' => board_columns::G,
    //             'H' => board_columns::H,
    //             _ => return Err("no good"),
    //         };
    //         let row: usize = chars[1].to_digit(10).unwrap() as usize;
    //         return Ok(Move::new(
    //             Position::new(row, column),
    //             None,
    //             None,
    //             PieceType::Pawn,
    //         ));
    //     } else if chars.len() > 2 {
    //         let piece_type = match chars[0].to_ascii_uppercase() {
    //             'R' => PieceType::Rook,
    //             'K' => PieceType::King,
    //             'N' => PieceType::Knight,
    //             'Q' => PieceType::Queen,
    //             'B' => PieceType::Bishop,
    //             _ => return Err("No good"),
    //         }
    //     }
    //     Ok(Move::new(Position::new(1, 1), None, None, PieceType::King))
    // }
}
