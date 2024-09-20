use std::collections::HashMap;

use crate::{
    piece::{Piece, PieceColour, PieceType},
    position::{Move, MoveResult, Position},
};

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
#[derive(PartialEq, Eq)]
enum SquareStatus {
    Occupied,
    OutsideBounds,
    Capturable,
    Free,
}

pub enum CastleDirection {
    KingSide,
    QueenSide,
}

pub struct Board {
    pieces_in_play: HashMap<Position, Piece>,
}

impl Board {
    pub fn test_positions(&self) {
        for (pos, piece) in &self.pieces_in_play {
            assert_eq!(*pos, piece.position);
        }
    }

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

        println!("\n{}", ascii_graphic);
    }

    fn move_was_en_passant(
        &self,
        new_position: Position,
        old_position: Position,
        removed_piece: &Option<Piece>,
    ) -> bool {
        if let Some(_) = removed_piece {
            return false;
        }
        if self.pieces_in_play.get(&new_position).unwrap().piece_type != PieceType::Pawn {
            return false;
        }
        if new_position.column == old_position.column {
            return false;
        }

        true
    }

    fn move_was_pawn_jump(
        &self,
        new_position: Position,
        old_position: Position,
        colour: PieceColour,
    ) -> bool {
        if colour == PieceColour::White {
            new_position.row == old_position.row + 2
                && self.pieces_in_play.get(&new_position).unwrap().piece_type == PieceType::Pawn
        } else {
            new_position.row == old_position.row - 2
                && self.pieces_in_play.get(&new_position).unwrap().piece_type == PieceType::Pawn
        }
    }

    fn find_moveable_pieces(
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

    fn undo_move(
        &mut self,
        new_position: Position,
        old_position: Position,
        removed_piece: Option<Piece>,
    ) {
        let mut piece_to_move: Piece = self.pieces_in_play.remove(&new_position).unwrap();
        piece_to_move.position = old_position;
        self.pieces_in_play.insert(old_position, piece_to_move);
        if let Some(removed_piece) = removed_piece {
            self.pieces_in_play
                .insert(removed_piece.position, removed_piece);
        }
    }

    fn is_castle_available(&self, direction: &CastleDirection, colour: PieceColour) -> bool {
        if self.is_in_check(colour) {
            return false;
        }

        // king is good
        let king_position: Position = self.find_king_position(colour);
        if let None = self.pieces_in_play.get(&king_position) {
            panic!("{:?}", king_position);
        }
        if self.pieces_in_play.get(&king_position).unwrap().special == false {
            return false;
        }

        //rook is good
        let rook_position: Position = match (&direction, colour) {
            (CastleDirection::KingSide, PieceColour::White) => Position::new(1, board_columns::H),
            (CastleDirection::KingSide, PieceColour::Black) => Position::new(8, board_columns::H),
            (CastleDirection::QueenSide, PieceColour::White) => Position::new(1, board_columns::A),
            (CastleDirection::QueenSide, PieceColour::Black) => Position::new(8, board_columns::A),
        };
        if let Some(piece) = self.pieces_in_play.get(&rook_position) {
            if piece.piece_type != PieceType::Rook {
                return false;
            }
            if piece.special == false {
                return false;
            }
        } else {
            return false;
        }

        let row: usize = match colour {
            PieceColour::White => 1,
            PieceColour::Black => 8,
        };

        let first_column: usize = match &direction {
            CastleDirection::KingSide => king_position.column + 1,
            CastleDirection::QueenSide => board_columns::C,
        };

        let last_column: usize = match &direction {
            CastleDirection::KingSide => board_columns::H,
            CastleDirection::QueenSide => king_position.column,
        };

        let enemy_colour: PieceColour = match colour {
            PieceColour::Black => PieceColour::White,
            PieceColour::White => PieceColour::Black,
        };

        let moves: Vec<Position> = self.find_all_moves(enemy_colour);

        for column in first_column + 1..last_column {
            if let Some(_) = self.pieces_in_play.get(&Position::new(row, column)) {
                return false;
            }

            if moves.contains(&Position::new(row, column)) {
                return false;
            }
        }
        true
    }

    fn has_valid_move(&mut self, player: PieceColour) -> bool {
        let mut moves_to_check: Vec<(Position, Position, PieceColour)> = Vec::new();

        for piece in self.pieces_in_play.values() {
            if piece.colour != player {
                continue;
            }
            let ally_moves: Vec<Position> = self.find_moves(piece);
            for new_position in ally_moves {
                moves_to_check.push((new_position, piece.position, piece.colour));
            }
        }

        if self.is_castle_available(&CastleDirection::KingSide, player)
            || self.is_castle_available(&CastleDirection::QueenSide, player)
        {
            return true;
        }

        if moves_to_check.is_empty() {
            return false;
        }

        for (new_position, old_position, colour) in moves_to_check {
            if self.move_can_be_played(old_position, new_position, colour) {
                return true;
            }
        }

        false
    }

    pub fn is_stalemate(&mut self, player: PieceColour) -> bool {
        if self.is_in_check(player) {
            return false;
        }

        if self.has_valid_move(player) {
            return false;
        }

        true
    }

    // ? hope it works
    pub fn is_mate(&mut self, player: PieceColour) -> bool {
        if !self.is_in_check(player) {
            return false;
        }

        if self.has_valid_move(player) {
            return false;
        }

        true
    }

    pub fn is_promotion_available(&self, colour: PieceColour, movement: &Move) -> bool {
        if self
            .pieces_in_play
            .get(&movement.new_position)
            .unwrap()
            .piece_type
            != PieceType::Pawn
        {
            false
        } else if colour == PieceColour::White && movement.new_position.row == 8 {
            true
        } else if colour == PieceColour::Black && movement.new_position.row == 1 {
            true
        } else {
            false
        }
    }

    fn move_can_be_played(
        &mut self,
        old_position: Position,
        new_position: Position,
        colour: PieceColour,
    ) -> bool {
        let mut valid: bool = true;
        let mut piece_to_move: Piece = self.pieces_in_play.remove(&old_position).unwrap();
        piece_to_move.position = new_position;
        let mut removed_piece: Option<Piece> =
            self.pieces_in_play.insert(new_position, piece_to_move);
        if self.move_was_en_passant(new_position, old_position, &removed_piece) {
            removed_piece = self.handle_en_passant(new_position, old_position);
        }
        if self.is_in_check(colour) {
            valid = false;
        }
        self.undo_move(new_position, old_position, removed_piece);
        valid
    }

    fn find_king_position(&self, colour: PieceColour) -> Position {
        let mut king_position: Position = Position::new(0, 0);
        for piece in self.pieces_in_play.values() {
            if piece.colour == colour && piece.piece_type == PieceType::King {
                king_position = piece.position;
                break;
            }
        }
        // println!("{} {}", king_position.row, king_position.column);
        king_position
    }

    fn is_in_check(&self, player: PieceColour) -> bool {
        let king_position: Position = self.find_king_position(player);
        let enemy_colour: PieceColour = match player {
            PieceColour::Black => PieceColour::White,
            PieceColour::White => PieceColour::Black,
        };
        let enemy_moves: Vec<Position> = self.find_all_moves(enemy_colour);

        enemy_moves.contains(&king_position)
    }

    fn find_all_moves(&self, colour: PieceColour) -> Vec<Position> {
        let mut moves: Vec<Position> = Vec::new();
        for piece in self.pieces_in_play.values() {
            if piece.colour == colour {
                let new_moves = self.find_moves(piece);
                moves.extend(new_moves);
            }
        }
        moves
    }

    pub fn make_move(&mut self, colour: PieceColour, movement: &Move) -> MoveResult {
        let started_in_check: bool = self.is_in_check(colour);

        // finds possible pieces to move
        let pieces: Vec<&Piece> =
            match self.find_moveable_pieces(movement.piece_type, colour, &movement) {
                Some(p) => p,
                None => return MoveResult::MissingPiece,
            };

        let mut found: bool = false;
        let mut piece_to_move: &Piece = pieces[0];

        // checks if exactly one piece can be moved and gets that piece
        for piece in pieces {
            let moves: Vec<Position> = self.find_moves(piece);
            if !moves.contains(&movement.new_position) {
                continue;
            }
            if found {
                return MoveResult::AmbiguousMove;
            }
            found = true;
            piece_to_move = piece;
        }

        if found == false {
            return MoveResult::ImpossibleMove;
        }

        let old_position: Position = piece_to_move.position;
        // pulls piece from its old position
        let mut piece_to_move: Piece = self.pieces_in_play.remove(&old_position).unwrap();
        // update its position
        piece_to_move.position = movement.new_position;
        // get the captured piece if any and puts piece on teh new position
        let mut removed_piece: Option<Piece> = self
            .pieces_in_play
            .insert(piece_to_move.position, piece_to_move);
        if self.move_was_en_passant(movement.new_position, old_position, &removed_piece) {
            removed_piece = self.handle_en_passant(movement.new_position, old_position);
        }
        // if in check after move undo the move
        if self.is_in_check(colour) {
            self.undo_move(movement.new_position, old_position, removed_piece);
            if started_in_check || movement.piece_type == PieceType::King {
                return MoveResult::Checked;
            } else {
                return MoveResult::PiecePinned;
            }
        }

        // en passant no longer available
        for piece in self.pieces_in_play.values_mut() {
            if piece.piece_type == PieceType::Pawn {
                piece.special = false;
            }
        }

        if self.move_was_pawn_jump(movement.new_position, old_position, colour) {
            self.pieces_in_play
                .get_mut(&movement.new_position)
                .unwrap()
                .special = true;
        } else {
            self.pieces_in_play
                .get_mut(&movement.new_position)
                .unwrap()
                .special = false;
        }

        if self.is_promotion_available(colour, movement) {
            MoveResult::PromotionAvailable(movement.new_position)
        } else {
            MoveResult::Success
        }
    }

    pub fn castle(&mut self, colour: PieceColour, direction: CastleDirection) -> MoveResult {
        if !self.is_castle_available(&direction, colour) {
            return MoveResult::ImpossibleMove;
        }

        let king_position: Position = self.find_king_position(colour);
        let rook_position: Position = match (&direction, colour) {
            (CastleDirection::KingSide, PieceColour::White) => Position::new(1, board_columns::H),
            (CastleDirection::KingSide, PieceColour::Black) => Position::new(8, board_columns::H),
            (CastleDirection::QueenSide, PieceColour::White) => Position::new(1, board_columns::A),
            (CastleDirection::QueenSide, PieceColour::Black) => Position::new(8, board_columns::A),
        };

        let king_new_position: Position = match &direction {
            CastleDirection::KingSide => Position::new(king_position.row, king_position.column + 2),
            CastleDirection::QueenSide => {
                Position::new(king_position.row, king_position.column - 2)
            }
        };
        let rook_new_position: Position = match &direction {
            CastleDirection::KingSide => {
                Position::new(rook_position.row, king_new_position.column - 1)
            }
            CastleDirection::QueenSide => {
                Position::new(rook_position.row, king_new_position.column + 1)
            }
        };
        let mut rook: Piece = self.pieces_in_play.remove(&rook_position).unwrap();
        let mut king: Piece = self.pieces_in_play.remove(&king_position).unwrap();
        king.special = false;
        rook.special = false;
        king.position = king_new_position;
        rook.position = rook_new_position;
        self.pieces_in_play.insert(king_new_position, king);
        self.pieces_in_play.insert(rook_new_position, rook);

        MoveResult::Success
    }

    /// Be sure to check it the moved piece was a pawn!
    ///
    /// Will make illegal captures if called after a non-pawn piece is moved.
    ///
    /// Returns the captured pawn.
    fn handle_en_passant(
        &mut self,
        new_position: Position,
        old_position: Position,
    ) -> Option<Piece> {
        let mut captured_pawn: Option<Piece> = None;
        if new_position.column != old_position.column {
            captured_pawn = self
                .pieces_in_play
                .remove(&Position::new(old_position.row, new_position.column));
        }
        captured_pawn
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

    fn pawn_has_en_passant_left(&self, pawn: &Piece, step: i32) -> bool {
        if pawn.colour == PieceColour::White && pawn.position.column == board_columns::A {
            return false;
        } else if pawn.colour == PieceColour::Black && pawn.position.column == board_columns::H {
            return false;
        }

        let left_position: Position = Position::new(pawn.position.row, pawn.position.column - 1);
        let back_position: Position = Position::new(
            (pawn.position.row as i32 + step) as usize,
            left_position.column,
        );

        match self.validate_square(back_position, pawn.colour) {
            SquareStatus::Free => (),
            _ => return false,
        }

        if let Some(piece) = self.pieces_in_play.get(&left_position) {
            if piece.piece_type != PieceType::Pawn {
                return false;
            }

            if piece.special == false {
                return false;
            }
        } else {
            return false;
        }

        true
    }

    fn pawn_has_en_passant_right(&self, pawn: &Piece, step: i32) -> bool {
        if pawn.colour == PieceColour::White && pawn.position.column == board_columns::H {
            return false;
        } else if pawn.colour == PieceColour::Black && pawn.position.column == board_columns::A {
            return false;
        }
        let right_position: Position = Position::new(pawn.position.row, pawn.position.column + 1);
        let back_position: Position = Position::new(
            (pawn.position.row as i32 + step) as usize,
            right_position.column,
        );

        match self.validate_square(back_position, pawn.colour) {
            SquareStatus::Free => (),
            _ => return false,
        }

        if let Some(piece) = self.pieces_in_play.get(&right_position) {
            if piece.piece_type != PieceType::Pawn {
                return false;
            }

            if piece.special == false {
                return false;
            }
        } else {
            return false;
        }

        true
    }

    fn pawn_has_jump(&self, pawn: &Piece) -> bool {
        if pawn.colour == PieceColour::White && pawn.position.row == 2 {
            if let SquareStatus::Free = self.validate_square(
                Position::new(pawn.position.row + 2, pawn.position.column),
                pawn.colour,
            ) {
                return true;
            }
        } else if pawn.colour == PieceColour::Black && pawn.position.row == 7 {
            if let SquareStatus::Free = self.validate_square(
                Position::new(pawn.position.row - 2, pawn.position.column),
                pawn.colour,
            ) {
                return true;
            }
        }

        false
    }

    fn find_pawn_moves(&self, moves: &mut Vec<Position>, piece: &Piece) {
        // TODO refactor en passant functions into one
        let mut current_position: Position = piece.position;
        match (current_position.row, piece.colour) {
            (8, PieceColour::White) => return,
            (1, PieceColour::Black) => return,
            _ => (),
        };

        let step: i32 = match &piece.colour {
            PieceColour::White => 1,
            PieceColour::Black => -1,
        };
        current_position.row = (current_position.row as i32 + step) as usize;
        if let SquareStatus::Free = self.validate_square(current_position, piece.colour) {
            moves.push(current_position);
        }

        if self.pawn_has_jump(piece) {
            current_position.row = (current_position.row as i32 + step) as usize;
            moves.push(current_position);
        }

        let back_left_position = Position::new(
            (piece.position.row as i32 + step) as usize,
            piece.position.column - 1,
        );

        let back_right_position: Position = Position::new(
            (piece.position.row as i32 + step) as usize,
            piece.position.column + 1,
        );

        if self.pawn_has_en_passant_left(piece, step) {
            // let back_position: Position = Position::new(
            //     (piece.position.row as i32 + step) as usize,
            //     piece.position.column - 1,
            // );
            moves.push(back_left_position);
        }

        if self.pawn_has_en_passant_right(piece, step) {
            // let back_position: Position = Position::new(
            //     (piece.position.row as i32 + step) as usize,
            //     piece.position.column + 1,
            // );
            moves.push(back_right_position);
        }

        if let SquareStatus::Capturable = self.validate_square(back_left_position, piece.colour) {
            moves.push(back_left_position);
        }

        if let SquareStatus::Capturable = self.validate_square(back_right_position, piece.colour) {
            moves.push(back_right_position);
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

    pub fn promote(&mut self, position: Position, colour: PieceColour, piece_type: PieceType) {
        let new_piece: Piece = Piece::new(piece_type, position.column, position.row, colour, false);
        self.pieces_in_play.insert(position, new_piece);
    }
}
