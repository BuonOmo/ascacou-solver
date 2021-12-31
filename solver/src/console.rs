
use ascacou_solver::game::Game;
use ascacou_solver::board::Board;

fn main() {
    for i in 0..=9 {
        for j in 0..=9 {
            print!("\x1b[4{};3{}m ●", i, j);
        }
        println!(" \x1b[0m");
    }

    Game::run_new(
        std::env::args().nth(1).and_then(|str| Board::from_fen(&str).ok())
    );
}
