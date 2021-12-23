mod board;
mod color;
mod mov;
mod player;
mod solver;

fn main() {
    for i in 0..=9 {
        for j in 0..=9 {
            print!("\x1b[4{};3{}m ●", i, j);
        }
        println!(" \x1b[0m");
    }
    // println!("\x1b[30m● \x1b[31m● \x1b[32m● \x1b[33m● \x1b[34m● \x1b[35m● \x1b[36m● \x1b[37m●");
}
