#![feature(gen_blocks)]
#![feature(test)]

extern crate test;

pub(crate) mod board;
pub(crate) mod color;
pub(crate) mod game;
pub(crate) mod mov;
pub(crate) mod player;
pub(crate) mod tileset;

pub use board::Board;
pub use color::Color;
pub use game::Game;
pub use mov::Move;

#[doc = include_str!("../../docs/Game-Analysis.md")]
#[cfg(doctest)]
pub struct GameAnalysisDoctests;
