use board::Board;
use game::GameManager;
mod board;
mod game;
mod piece;
mod position;

fn main() {
    let board: Board = Board::new();
    let mut game: GameManager = GameManager::new(board);
    game.start_game();
}
