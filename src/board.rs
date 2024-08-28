use crate::{
    piece::{self, Piece, PieceColour, PieceType},
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
    pieces_in_play: Vec<piece::Piece>,
    pub board_state: [[Option<Piece>; 9]; 9],
}

impl Board {
    pub fn new() -> Board {
        use piece::Piece;
        use piece::PieceColour;
        use piece::PieceType;

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

        let mut squares: [[Option<Piece>; 9]; 9] = [[None; 9]; 9];
        let mut pieces: Vec<piece::Piece> = Vec::new();

        for (i, &piece_type) in back_types.iter().enumerate() {
            let special: bool = match piece_type {
                PieceType::King | PieceType::Rook => true,
                _ => false,
            };
            pieces.push(Piece::new(
                piece_type,
                board_columns::A + i,
                1,
                PieceColour::White,
                special,
            ));

            pieces.push(Piece::new(
                piece_type,
                board_columns::A + i,
                8,
                PieceColour::Black,
                special,
            ));

            squares[1][board_columns::A + i] = Some(Piece::new(
                piece_type,
                board_columns::A + i,
                1,
                PieceColour::White,
                special,
            ));

            squares[8][board_columns::A + i] = Some(Piece::new(
                piece_type,
                board_columns::A + i,
                8,
                PieceColour::Black,
                special,
            ));
        }

        for column in board_columns::A..=board_columns::H {
            pieces.push(piece::Piece::new(
                piece::PieceType::Pawn,
                column,
                2,
                piece::PieceColour::White,
                false,
            ));

            pieces.push(piece::Piece::new(
                piece::PieceType::Pawn,
                column,
                7,
                piece::PieceColour::Black,
                false,
            ));

            squares[2][column as usize] = Some(piece::Piece::new(
                piece::PieceType::Pawn,
                column,
                2,
                piece::PieceColour::White,
                false,
            ));

            squares[7][column as usize] = Some(piece::Piece::new(
                piece::PieceType::Pawn,
                column,
                7,
                piece::PieceColour::Black,
                false,
            ));
        }

        Board {
            pieces_in_play: pieces,
            board_state: squares,
        }
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
                if let Some(p) = self.board_state[row][column] {
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

    fn index_from_position(&self, position: Position) -> Option<usize> {
        for (index, piece) in self.pieces_in_play.iter().enumerate() {
            if piece.position == position {
                return Some(index);
            }
        }
        None
    }

    pub fn find_piece_index(
        &self,
        piece_type: PieceType,
        colour: PieceColour,
        movement: &Move,
    ) -> Result<Vec<usize>, &'static str> {
        let mut piece_indexes: Vec<usize> = Vec::new();
        for (index, piece) in self.pieces_in_play.iter().enumerate() {
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

            piece_indexes.push(index);
        }

        if piece_indexes.is_empty() {
            Err("piece not found")
        } else {
            Ok(piece_indexes)
        }
    }

    fn handle_capture(&mut self, movement: &Move, piece_index: usize) {
        let captured_piece_index: Option<usize>;
        let moved_piece: &Piece = &self.pieces_in_play[piece_index];
        if moved_piece.piece_type == PieceType::Pawn
            && movement.end_position.column != moved_piece.position.column
        {
            captured_piece_index = self.index_from_position(Position::new(
                moved_piece.position.row,
                movement.end_position.column,
            ));
        } else {
            captured_piece_index = self.index_from_position(movement.end_position)
        }
        if let Some(index) = captured_piece_index {
            let capture_position: Position = self.pieces_in_play[index].position;
            self.board_state[capture_position.row][capture_position.column] = None;
            self.pieces_in_play.remove(index);
        }
    }

    pub fn make_move(
        &mut self,
        piece_type: PieceType,
        colour: PieceColour,
        movement: Move,
    ) -> Result<(), &'static str> {
        let piece_indexes: Vec<usize> = self.find_piece_index(piece_type, colour, &movement)?;
        let mut found: bool = false;
        let mut piece_index: usize = 0;

        for index in piece_indexes {
            let piece: &Piece = &self.pieces_in_play[index];
            let moves: Vec<Position> = self.find_moves(piece);
            // TODO add Check validation
            if moves.contains(&movement.end_position) {
                if !found {
                    found = true;
                    piece_index = index;
                } else {
                    return Err("multiple pieces can make this move");
                }
            }
        }
        if found == false {
            return Err("impossible move");
        }
        let old_position: Position = self.pieces_in_play[piece_index].position;
        let special: bool = self.pieces_in_play[piece_index].special;
        self.handle_capture(&movement, piece_index);
        self.board_state[old_position.row][old_position.column] = None;
        self.pieces_in_play[piece_index].position = movement.end_position;
        self.board_state[movement.end_position.row][movement.end_position.column] =
            Some(Piece::new(
                piece_type,
                movement.end_position.column,
                movement.end_position.row,
                colour,
                special,
            ));

        piece_index = self.index_from_position(movement.end_position).unwrap();
        let moved_piece: &mut Piece = &mut self.pieces_in_play[piece_index];
        if piece_type != PieceType::Pawn {
            moved_piece.special = false;
        } else {
            if movement.end_position.row - old_position.row == 2
                && movement.end_position.column == old_position.column
            {
                moved_piece.special = true;
            }
        }
        Ok(())
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
                if let Some(side_piece) = self.board_state[side_position.row][side_position.column]
                {
                    if side_piece.piece_type == PieceType::Pawn {
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
            match self.board_state[position.row][position.column] {
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
