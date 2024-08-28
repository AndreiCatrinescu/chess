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

#[derive(Clone, Copy)]
#[allow(dead_code)]
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
        let mut symbol: char = match piece_type {
            PieceType::King => 'K',
            PieceType::Bishop => 'B',
            PieceType::Knight => 'N',
            PieceType::Pawn => 'P',
            PieceType::Queen => 'Q',
            PieceType::Rook => 'R',
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
