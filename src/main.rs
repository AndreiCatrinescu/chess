use game::GameManager;
mod board;
mod game;
mod piece;
mod position;
mod timer;

fn main() {
    let mut game: GameManager = GameManager::new();
    game.start_game();
}
