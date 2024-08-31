use std::collections::HashMap;

use crate::{
    piece::{Piece, PieceColour, PieceType},
    position::{Move, Position},
};

#[allow(dead_code)]
pub mod board_columns {
    pub const A: usize = 1;
    pub const B: usize = 2;
    pub const C: usize = 3;
    pub const D: usize = 4;
    pub const E: usize = 5;
    pub const F: usize = 6;
    pub const G: usize = 7;
    pub const H: usize = 8;
}

enum SquareStatus {
    Occupied,
    OutsideBounds,
    Capturable,
    Free,
}

pub struct Board {
    pieces_in_play: HashMap<Position, Piece>,
}

impl Board {
    pub fn new() -> Self {
        let back_types: [PieceType; 8] = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        let mut pieces_in_play: HashMap<Position, Piece> = HashMap::new();

        for (i, &piece_type) in back_types.iter().enumerate() {
            let special: bool = match piece_type {
                PieceType::King | PieceType::Rook => true,
                _ => false,
            };

            let new_white_piece: Piece = Piece::new(
                piece_type,
                board_columns::A + i,
                1,
                PieceColour::White,
                special,
            );

            let new_black_piece: Piece = Piece::new(
                piece_type,
                board_columns::A + i,
                8,
                PieceColour::Black,
                special,
            );

            let new_white_position: Position = Position::new(1, board_columns::A + i);
            let new_black_position: Position = Position::new(8, board_columns::A + i);

            pieces_in_play.insert(new_white_position, new_white_piece);
            pieces_in_play.insert(new_black_position, new_black_piece);
        }

        for column in board_columns::A..=board_columns::H {
            let new_white_pawn: Piece =
                Piece::new(PieceType::Pawn, column, 2, PieceColour::White, false);
            let new_black_pawn: Piece =
                Piece::new(PieceType::Pawn, column, 7, PieceColour::Black, false);

            let new_white_position = Position::new(2, column);
            let new_black_position = Position::new(7, column);

            pieces_in_play.insert(new_white_position, new_white_pawn);
            pieces_in_play.insert(new_black_position, new_black_pawn);
        }

        Board { pieces_in_play }
    }

    pub fn print(&self) {
        let mut ascii_graphic: String = String::new();
        let letters: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        ascii_graphic.push_str("  ");
        for _ in 1..=16 {
            ascii_graphic.push('_');
        }
        ascii_graphic.push('\n');

        for row in (1..=8).rev() {
            ascii_graphic.push_str(&row.to_string());
            ascii_graphic.push('|');

            for column in 1..=8 {
                if let Some(p) = self.pieces_in_play.get(&Position::new(row, column)) {
                    ascii_graphic.push_str(&format!("{} ", p.symbol));
                } else if (row % 2) + (column % 2) == 1 {
                    ascii_graphic.push_str("  ");
                } else {
                    ascii_graphic.push_str("██");
                }
            }

            ascii_graphic.push('|');
            ascii_graphic.push('\n');
        }
        ascii_graphic.push_str("  ");

        for _ in 1..=16 {
            ascii_graphic.push('‾');
        }
        ascii_graphic.push('\n');
        ascii_graphic.push_str("  ");

        for letter in letters {
            ascii_graphic.push(letter);
            ascii_graphic.push(' ');
        }
        ascii_graphic.push('\n');

        println!("{}", ascii_graphic);
    }

    pub fn find_moveable_pieces(
        &self,
        piece_type: PieceType,
        colour: PieceColour,
        movement: &Move,
    ) -> Option<Vec<&Piece>> {
        let mut pieces: Vec<&Piece> = Vec::new();
        for piece in self.pieces_in_play.values() {
            if piece.colour != colour || piece.piece_type != piece_type {
                continue;
            }

            let is_on_starting_position: bool =
                match (movement.starting_column, movement.starting_row) {
                    (Some(column), Some(row)) => piece.position == Position::new(row, column),
                    (Some(column), None) => piece.position.column == column,
                    (None, Some(row)) => piece.position.row == row,
                    (None, None) => true,
                };

            if !is_on_starting_position {
                continue;
            }

            pieces.push(piece);
        }

        if pieces.is_empty() {
            None
        } else {
            Some(pieces)
        }
    }

    pub fn make_move(
        &mut self,
        piece_type: PieceType,
        colour: PieceColour,
        movement: Move,
    ) -> Result<(), &'static str> {
        let pieces: Vec<&Piece> = match self.find_moveable_pieces(piece_type, colour, &movement) {
            Some(p) => p,
            None => return Err("piece not found"),
        }; // imm

        let mut found: bool = false;
        let mut piece_to_move: &Piece = pieces[0];

        for piece in pieces {
            let moves: Vec<Position> = self.find_moves(piece);
            // TODO add Check validation
            if !moves.contains(&movement.end_position) {
                continue;
            }
            if found {
                return Err("multiple pieces can make this move");
            }
            found = true;
            piece_to_move = piece;
        }
        if found == false {
            return Err("impossible move");
        }
        let old_position: Position = piece_to_move.position;

        let mut piece_to_move: Piece = self.pieces_in_play.remove(&old_position).unwrap();
        piece_to_move.position = movement.end_position;

        self.pieces_in_play
            .insert(piece_to_move.position, piece_to_move); // handles captures other than en passant automatically
        self.handle_en_passant(&movement, old_position);

        Ok(())
    }

    fn handle_en_passant(&mut self, movement: &Move, old_position: Position) {
        let moved_piece: &Piece = self.pieces_in_play.get(&movement.end_position).unwrap();
        if let PieceType::Pawn = moved_piece.piece_type {
            if movement.end_position.column != old_position.column {
                self.pieces_in_play.remove(&Position::new(
                    old_position.row,
                    movement.end_position.column,
                ));
            }
        }
    }

    fn find_moves(&self, piece: &Piece) -> Vec<Position> {
        let mut moves: Vec<Position> = Vec::new();
        match piece.piece_type {
            PieceType::Bishop => self.find_bishop_moves(&mut moves, piece),
            PieceType::King => self.find_king_moves(&mut moves, piece),
            PieceType::Queen => self.find_queen_moves(&mut moves, piece),
            PieceType::Knight => self.find_knight_moves(&mut moves, piece),
            PieceType::Pawn => self.find_pawn_moves(&mut moves, piece),
            PieceType::Rook => self.find_rook_moves(&mut moves, piece),
        };
        moves
    }

    fn find_bishop_moves(&self, moves: &mut Vec<Position>, piece: &Piece) {
        let directions: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        for direction in directions {
            let mut current_position: Position = piece.position;
            loop {
                current_position.column = (current_position.column as i32 + direction.0) as usize;
                current_position.row = (current_position.row as i32 + direction.1) as usize;

                match self.validate_square(current_position, piece.colour) {
                    SquareStatus::OutsideBounds | SquareStatus::Occupied => break,
                    SquareStatus::Capturable => {
                        moves.push(current_position);
                        break;
                    }
                    SquareStatus::Free => moves.push(current_position),
                }
            }
        }
    }

    fn find_king_moves(&self, moves: &mut Vec<Position>, piece: &Piece) {
        let directons: [(i32, i32); 8] = [
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        for direction in directons {
            let mut current_position: Position = piece.position;
            current_position.column = (current_position.column as i32 + direction.0) as usize;
            current_position.row = (current_position.row as i32 + direction.1) as usize;
            match self.validate_square(current_position, piece.colour) {
                SquareStatus::OutsideBounds | SquareStatus::Occupied => (),
                SquareStatus::Capturable | SquareStatus::Free => moves.push(current_position),
            }
        }
    }

    fn find_queen_moves(&self, moves: &mut Vec<Position>, piece: &Piece) {
        self.find_bishop_moves(moves, piece);
        self.find_rook_moves(moves, piece);
    }

    fn find_knight_moves(&self, moves: &mut Vec<Position>, piece: &Piece) {
        let directons: [(i32, i32); 8] = [
            (2, 1),
            (2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
            (-2, 1),
            (-2, -1),
        ];

        for direction in directons {
            let mut current_position: Position = piece.position;
            current_position.column = (current_position.column as i32 + direction.0) as usize;
            current_position.row = (current_position.row as i32 + direction.1) as usize;
            match self.validate_square(current_position, piece.colour) {
                SquareStatus::OutsideBounds | SquareStatus::Occupied => (),
                SquareStatus::Capturable | SquareStatus::Free => moves.push(current_position),
            }
        }
    }

    fn find_pawn_moves(&self, moves: &mut Vec<Position>, piece: &Piece) {
        // TODO solve en passant
        // TODO rewrite maybe
        let mut current_position: Position = piece.position;
        if piece.colour == PieceColour::White {
            current_position.row += 1;
            if let SquareStatus::Free = self.validate_square(current_position, piece.colour) {
                // One step up
                moves.push(current_position);
            }
            if piece.position.row == 2 {
                // Two steps up
                current_position.row += 1;
                if let SquareStatus::Free = self.validate_square(current_position, piece.colour) {
                    moves.push(current_position);
                }
            }
            let mut jumped: bool = false;
            // En passant left
            let mut back_position: Position = piece.position;
            let mut side_position: Position = piece.position;
            back_position.row += 1;
            back_position.column -= 1;
            side_position.column -= 1;
            if let SquareStatus::Capturable = self.validate_square(side_position, piece.colour) {
                if let Some(side_piece) = self.pieces_in_play.get(&side_position) {
                    if let PieceType::Pawn = side_piece.piece_type {
                        jumped = side_piece.special;
                    }
                }
            }

            if let SquareStatus::Free = self.validate_square(back_position, piece.colour) {
                if jumped {
                    moves.push(back_position);
                }
            }
            // En passant right
            back_position = piece.position;
            back_position.row += 1;
            back_position.column += 1;
            side_position = piece.position;
            side_position.column += 1;
        } else {
            current_position.row -= 1;
            if let SquareStatus::Free = self.validate_square(current_position, piece.colour) {
                // One step down
                moves.push(current_position);
            }
            if piece.position.row == 7 {
                // Two steps down
                current_position.row -= 1;
                if let SquareStatus::Free = self.validate_square(current_position, piece.colour) {
                    moves.push(current_position);
                }
            }
        }
    }

    fn find_rook_moves(&self, moves: &mut Vec<Position>, piece: &Piece) {
        let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for direction in directions {
            let mut current_position: Position = piece.position;
            loop {
                current_position.column = (current_position.column as i32 + direction.0) as usize;
                current_position.row = (current_position.row as i32 + direction.1) as usize;

                match self.validate_square(current_position, piece.colour) {
                    SquareStatus::OutsideBounds | SquareStatus::Occupied => break,
                    SquareStatus::Capturable => {
                        moves.push(current_position);
                        break;
                    }
                    SquareStatus::Free => moves.push(current_position),
                }
            }
        }
    }

    fn validate_square(&self, position: Position, ally_colour: PieceColour) -> SquareStatus {
        if position.column > board_columns::H || position.column < board_columns::A {
            SquareStatus::OutsideBounds
        } else if position.row > 8 || position.row < 1 {
            SquareStatus::OutsideBounds
        } else {
            match self.pieces_in_play.get(&position) {
                Some(piece) => {
                    if piece.colour == ally_colour {
                        SquareStatus::Occupied
                    } else {
                        SquareStatus::Capturable
                    }
                }
                None => SquareStatus::Free,
            }
        }
    }
}
