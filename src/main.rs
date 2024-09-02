mod board;
mod game;
mod piece;
mod position;
use board::board_columns;
use position::{Move, Position};

fn main() {
    // TODO test en passant
    // let mut t: board::Board = board::Board::new();
    // let m: Move = Move {
    //     end_position: Position::new(6, board_columns::H),
    //     starting_column: None,
    //     starting_row: None,
    // };
    // let m2: Move = Move {
    //     end_position: Position::new(1, board_columns::C),
    //     starting_column: None,
    //     starting_row: None,
    // };
    // let m3: Move = Move {
    //     end_position: Position::new(3, board_columns::A),
    //     starting_column: None,
    //     starting_row: None,
    // };
    // let m4: Move = Move {
    //     end_position: Position::new(1, board_columns::C),
    //     starting_column: None,
    //     starting_row: None,
    // };
    // t.print();
    // t.make_move(piece::PieceType::Bishop, piece::PieceColour::White, m)
    //     .unwrap();
    // t.print();
    // t.make_move(piece::PieceType::Bishop, piece::PieceColour::White, m2)
    //     .unwrap();
    // t.print();
    // t.make_move(piece::PieceType::Bishop, piece::PieceColour::White, m3)
    //     .unwrap();
    // t.print();
    // t.make_move(piece::PieceType::Bishop, piece::PieceColour::White, m4)
    //     .unwrap();
    // t.print();
}
