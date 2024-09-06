use crate::{
    board::{Board, CastleDirection},
    piece::{PieceColour, PieceType},
    position::{Move, MoveResult, Position},
};
use std::io;
pub struct GameManager {
    board: Board,
    turn: PieceColour,
}

impl GameManager {
    pub fn new(board: Board) -> Self {
        GameManager {
            board,
            turn: PieceColour::White,
        }
    }

    pub fn start_game(&mut self) {
        let input: io::Stdin = io::stdin();
        let mut move_notation: String = String::new();
        let mut move_result: MoveResult;
        let mut movement: Move;
        self.board.print();
        loop {
            println!("your move:");
            move_notation.clear();
            input.read_line(&mut move_notation).unwrap();
            move_notation = move_notation.trim().to_string();
            if move_notation.to_ascii_lowercase() == "o-o" {
                move_result = self.board.castle(self.turn, CastleDirection::KingSide);
            } else if move_notation.to_ascii_lowercase() == "o-o-o" {
                move_result = self.board.castle(self.turn, CastleDirection::QueenSide);
            } else if move_notation.to_ascii_lowercase() == "exit" {
                break;
            } else {
                movement = match Move::from_notation(&move_notation) {
                    Ok(movement) => movement,
                    Err(_) => {
                        println!("notation is invalid");
                        continue;
                    }
                };
                move_result = self.board.make_move(self.turn, &movement);

                if let MoveResult::PromotionAvailable = move_result {
                    println!("promotion available");
                    self.halde_promotion(self.turn, movement.new_position);
                }
            }

            match move_result {
                MoveResult::AmbiguousMove => {
                    println!("multiple pieces can make this move, consider specifying the starting row or column");
                    continue;
                }
                MoveResult::Checked => {
                    println!("cannot make this move due to check");
                    continue;
                }
                MoveResult::ImpossibleMove => {
                    println!("this move is not legal");
                    continue;
                }
                MoveResult::MissingPiece => {
                    println!("no piece can make this move");
                    continue;
                }
                MoveResult::Success | MoveResult::PromotionAvailable => (),
            }

            self.turn = match &self.turn {
                PieceColour::Black => PieceColour::White,
                PieceColour::White => PieceColour::Black,
            };

            if self.board.is_mate(self.turn) {
                self.board.print();
                println!("mate");
                break;
            }

            self.board.print();
        }
    }

    fn halde_promotion(&mut self, turn: PieceColour, position: Position) {
        println!("choose piece to promote to:");
        let input: io::Stdin = io::stdin();
        let mut piece_string: String = String::new();
        loop {
            input.read_line(&mut piece_string).unwrap();
            piece_string = piece_string.to_ascii_lowercase();
            let piece_symbol: char = piece_string.chars().nth(0).unwrap();
            let piece_type: PieceType = match piece_symbol {
                'r' => PieceType::Rook,
                'q' => PieceType::Queen,
                'n' | 'k' => PieceType::Knight,
                'b' => PieceType::Bishop,
                _ => {
                    println!("no :)");
                    continue;
                }
            };
            self.board.promote(position, turn, piece_type);
            return;
        }
    }
}
