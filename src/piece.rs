use crate::position::Position;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceColour {
    Black,
    White,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub position: Position,
    pub symbol: char,
    pub colour: PieceColour,
    pub special: bool,
}

impl Piece {
    pub fn new(
        piece_type: PieceType,
        column: usize,
        row: usize,
        colour: PieceColour,
        special: bool,
    ) -> Piece {
        let mut symbol: char = match (piece_type, colour) {
            (PieceType::King, PieceColour::Black) => '♔',
            (PieceType::Bishop, PieceColour::Black) => '♗',
            (PieceType::Knight, PieceColour::Black) => '♘',
            (PieceType::Pawn, PieceColour::Black) => '♙',
            (PieceType::Queen, PieceColour::Black) => '♕',
            (PieceType::Rook, PieceColour::Black) => '♖',
            (PieceType::King, PieceColour::White) => '♚',
            (PieceType::Bishop, PieceColour::White) => '♝',
            (PieceType::Knight, PieceColour::White) => '♞',
            (PieceType::Pawn, PieceColour::White) => '♟',
            (PieceType::Queen, PieceColour::White) => '♛',
            (PieceType::Rook, PieceColour::White) => '♜',
        };

        if let PieceColour::Black = colour {
            symbol = symbol.to_lowercase().next().unwrap();
        }

        Piece {
            piece_type,
            colour,
            position: Position::new(row, column),
            symbol,
            special,
        }
    }
}
