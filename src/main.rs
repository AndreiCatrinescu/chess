use board::Board;
use game::GameManager;
mod board;
mod game;
mod piece;
mod position;
// use board::board_columns;
// use position::{Move, Position};

fn main() {
    let board: Board = Board::new();
    let mut game = GameManager::new(board);
    game.start_game();
}
