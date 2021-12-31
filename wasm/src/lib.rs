use ascacou_solver::solver::Solver;
use ascacou_solver::board::Board;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn solve(fen: &str, depth: u8) -> Option<String> {
    // let mut result = 128u8;
    // if let Ok(board) = Board::from_fen(fen) {
    //     if let (_, Some(mov), _) = Solver::solve(&board, Some(depth)) {
    //         result = 0; // No error possible from there.
    //         if mov.is_black() { result |= 64 }
    //         result |= (mov.x as u8) << 3;
    //         result |= mov.y as u8;
    //     }
    // }
    Board::from_fen(fen).ok().and_then(|board|{
        let (_, mov_opt, _) = Solver::solve(&board, Some(depth));
        mov_opt.and_then(|mov|Some(mov.into()))
    })
}
