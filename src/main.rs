use game::GameManager;
use std::env::args;
mod board;
mod game;
mod piece;
mod position;
mod timer;

fn main() {
    let game_duration: u64;
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        game_duration = 10;
    } else {
        game_duration = match args.get(1).unwrap().parse::<u64>() {
            Ok(ammount) => ammount,
            Err(_) => 10,
        }
    }
    let mut game: GameManager = GameManager::new(game_duration);
    game.start_game();
}
