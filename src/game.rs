use crate::{
    board::{Board, CastleDirection},
    piece::{PieceColour, PieceType},
    position::{Move, MoveResult, Position},
    timer::Timer,
};
use std::process;
use std::time::Instant;
use std::{io, sync::Arc};
use std::{io::Write, thread};
use std::{sync::Mutex, time::Duration};
pub struct GameManager {
    board: Board,
    turn: Arc<Mutex<PieceColour>>,
    white_timer: Arc<Timer>,
    black_timer: Arc<Timer>,
}

const GAME_TIME: u64 = 60 * 100;
impl GameManager {
    pub fn new() -> Self {
        GameManager {
            board: Board::new(),
            turn: Arc::new(Mutex::new(PieceColour::White)),
            white_timer: Arc::new(Timer::new(GAME_TIME)),
            black_timer: Arc::new(Timer::new(GAME_TIME)),
        }
    }

    fn update_timer(
        white_timer: Arc<Timer>,
        black_timer: Arc<Timer>,
        turn: Arc<Mutex<PieceColour>>,
    ) {
        const ESC: &str = "\x1B[";
        const PRECISION: Duration = Duration::from_millis(100);
        let mut start = Instant::now();
        loop {
            let end = Instant::now();
            if end - start >= PRECISION {
                print!("{}s", ESC);
                let current_turn = turn.lock().unwrap();
                let rem;
                match *current_turn {
                    PieceColour::White => {
                        if white_timer.is_finished() {
                            break;
                        }
                        print!("{}13;0H", ESC);
                        rem = white_timer.remaining_duration();
                    }
                    PieceColour::Black => {
                        if black_timer.is_finished() {
                            break;
                        }
                        print!("{}H", ESC);
                        rem = black_timer.remaining_duration();
                    }
                }
                drop(current_turn);
                print!("{}2K", ESC);
                println!(
                    "{}:{}.{}",
                    rem.as_secs() / 60,
                    rem.as_secs() % 60,
                    (rem.as_millis() - (rem.as_secs() * 1000) as u128) / 100
                );
                print!("\x1B[u");
                io::stdout().flush().unwrap();
                start = end;
            }
        }
        process::exit(0);
    }

    pub fn start_game(&mut self) {
        let input: io::Stdin = io::stdin();
        let mut move_notation: String = String::new();
        let mut move_result: MoveResult;
        let mut movement: Move;
        let w_timer_clone = Arc::clone(&self.white_timer);
        let b_timer_clone = Arc::clone(&self.black_timer);
        let turn_clone = Arc::clone(&self.turn);
        self.print();
        thread::spawn(move || {
            w_timer_clone.countdown_start();
            b_timer_clone.pause();
            b_timer_clone.countdown_start();
            GameManager::update_timer(w_timer_clone, b_timer_clone, turn_clone);
        });
        loop {
            // self.board.test_positions();
            let turn_lock = self.turn.lock().unwrap();
            let mut turn = *turn_lock;
            drop(turn_lock);
            println!("your move:");
            move_notation.clear();
            input.read_line(&mut move_notation).unwrap();
            move_notation = move_notation.trim().to_string();
            if move_notation.to_ascii_lowercase() == "o-o" {
                move_result = self.board.castle(turn, CastleDirection::KingSide);
            } else if move_notation.to_ascii_lowercase() == "o-o-o" {
                move_result = self.board.castle(turn, CastleDirection::QueenSide);
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
                move_result = self.board.make_move(turn, &movement);
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
                MoveResult::PiecePinned => {
                    println!("this piece is pinned");
                    continue;
                }
                MoveResult::PromotionAvailable(square) => self.handle_promotion(turn, square),
                MoveResult::Success => (),
            }

            let mut turn_lock = self.turn.lock().unwrap();
            *turn_lock = match *turn_lock {
                PieceColour::White => {
                    self.white_timer.pause();
                    self.black_timer.resume();
                    PieceColour::Black
                }
                PieceColour::Black => {
                    self.black_timer.pause();
                    self.white_timer.resume();
                    PieceColour::White
                }
            };

            turn = *turn_lock;
            drop(turn_lock);

            if self.board.is_stalemate(turn) {
                self.print();
                println!("stalemate");
                break;
            }

            if self.board.is_mate(turn) {
                self.print();
                println!("mate");
                break;
            }
            self.print();
        }
    }

    fn print(&self) {
        print!("\x1B[2J");
        print!("\x1B[H");
        let mut rem = self.black_timer.remaining_duration();
        println!(
            "{}:{}.{}",
            rem.as_secs() / 60,
            rem.as_secs() % 60,
            (rem.as_millis() - (rem.as_secs() * 1000) as u128) / 100
        );
        print!("\x1B[1;0H");
        self.board.print();
        rem = self.white_timer.remaining_duration();
        print!("\x1B[13;0H");
        io::stdout().flush().unwrap();
        println!(
            "{}:{}.{}",
            rem.as_secs() / 60,
            rem.as_secs() % 60,
            (rem.as_millis() - (rem.as_secs() * 1000) as u128) / 100
        );
    }

    fn handle_promotion(&mut self, turn: PieceColour, position: Position) {
        println!("choose piece to promote to:");
        let input: io::Stdin = io::stdin();
        let mut piece_string: String = String::new();
        loop {
            piece_string.clear();
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
