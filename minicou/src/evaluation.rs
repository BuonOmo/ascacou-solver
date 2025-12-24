use ascacou::{Board, TileSet};

/// Evaluation of a board is quite tricky. On
/// over the board games I can see that we tend
/// not to fill tiles too early, and wait until
/// the end. The forced move strategy for
/// evaluation is already a good start, but we
/// could do better by evaluating partial tile
/// completion.

pub fn evaluate(board: &Board) -> i16 {
	if board.played_moves < 2 {
		return 0;
	}
	if board.played_moves < 3 {
		return half_tiles(board) as i16;
	}
	if board.played_moves < 4 {
		return almost_full_tiles(board) as i16 + half_tiles(board) as i16;
	}
	if board.played_moves > 20 {
		return board.current_score() as i16 * 8;
	}
	let full_tiles_score = board.current_score() as i16;
	let almost_full_score = almost_full_tiles(board) as i16;
	let half_tiles_score = half_tiles(board) as i16;
	match board.played_moves {
		0..=8 => full_tiles_score + almost_full_score + half_tiles_score,
		9..=16 => full_tiles_score * 2 + almost_full_score * 2 + half_tiles_score,
		17..=20 => full_tiles_score * 4 + almost_full_score + half_tiles_score,
		_ => unreachable!("see guards"),
	}
}

fn almost_full_tiles(board: &Board) -> i8 {
	let x = board.pieces_mask;
	let bottom_right =
		MaskIterator(x & x >> 1 & x >> 7 & !(x >> 8)).fold(TileSet::empty(), |acc, half| {
			let top_left_black_presence = board.black_mask & half != 0;
			let top_right_black_presence = board.black_mask & half << 1 != 0;
			let bottom_left_black_presence = board.black_mask & half << 7 != 0;
			#[rustfmt::skip]
				let compatible_tiles = match (
					top_left_black_presence,
					top_right_black_presence,
					bottom_left_black_presence,
				) {
					/*   1,     2,     4 */
					(false, false, false) => TileSet::new(0b1000000010000000),
					( true, false, false) => TileSet::new(0b0100000001000000),
					(false,  true, false) => TileSet::new(0b0010000000100000),
					( true,  true, false) => TileSet::new(0b0001000000010000),
					(false, false,  true) => TileSet::new(0b0000100000001000),
					( true, false,  true) => TileSet::new(0b0000010000000100),
					(false,  true,  true) => TileSet::new(0b0000001000000010),
					( true,  true,  true) => TileSet::new(0b0000000100000001),
				};
			acc | compatible_tiles
		});

	let bottom_left =
		MaskIterator(x & x >> 1 & !(x >> 7) & x >> 8).fold(TileSet::empty(), |acc, half| {
			let top_left_black_presence = board.black_mask & half != 0;
			let top_right_black_presence = board.black_mask & half << 1 != 0;
			let bottom_right_black_presence = board.black_mask & half << 8 != 0;
			#[rustfmt::skip]
				let compatible_tiles = match (
					top_left_black_presence,
					top_right_black_presence,
					bottom_right_black_presence,
				) {
					/*   1,     2,     8 */
					(false, false, false) => TileSet::new(0b1000100000000000),
					( true, false, false) => TileSet::new(0b0100010000000000),
					(false,  true, false) => TileSet::new(0b0010001000000000),
					( true,  true, false) => TileSet::new(0b0001000100000000),
					(false, false,  true) => TileSet::new(0b0000000010001000),
					( true, false,  true) => TileSet::new(0b0000000001000100),
					(false,  true,  true) => TileSet::new(0b0000000000100010),
					( true,  true,  true) => TileSet::new(0b0000000000010001),
				};
			acc | compatible_tiles
		});

	let top_right =
		MaskIterator(x & !(x >> 1) & x >> 7 & x >> 8).fold(TileSet::empty(), |acc, half| {
			let top_left_black_presence = board.black_mask & half != 0;
			let bottom_left_black_presence = board.black_mask & half << 7 != 0;
			let bottom_right_black_presence = board.black_mask & half << 8 != 0;
			#[rustfmt::skip]
				let compatible_tiles = match (
					top_left_black_presence,
					bottom_left_black_presence,
					bottom_right_black_presence,
				) {
					/*   1,     4,     8 */
					(false, false, false) => TileSet::new(0b1010000000000000),
					( true, false, false) => TileSet::new(0b0101000000000000),
					(false,  true, false) => TileSet::new(0b0000101000000000),
					( true,  true, false) => TileSet::new(0b0000010100000000),
					(false, false,  true) => TileSet::new(0b0000000010100000),
					( true, false,  true) => TileSet::new(0b0000000001010000),
					(false,  true,  true) => TileSet::new(0b0000000000001010),
					( true,  true,  true) => TileSet::new(0b0000000000000101),
				};
			acc | compatible_tiles
		});

	let top_left =
		MaskIterator(!(x) & x >> 1 & x >> 7 & x >> 8).fold(TileSet::empty(), |acc, half| {
			let top_right_black_presence = board.black_mask & half << 1 != 0;
			let bottom_left_black_presence = board.black_mask & half << 7 != 0;
			let bottom_right_black_presence = board.black_mask & half << 8 != 0;
			#[rustfmt::skip]
				let compatible_tiles = match (
					top_right_black_presence,
					bottom_left_black_presence,
					bottom_right_black_presence,
				) {
					/*   2,     4,     8 */
					(false, false, false) => TileSet::new(0b1100000000000000),
					( true, false, false) => TileSet::new(0b0011000000000000),
					(false,  true, false) => TileSet::new(0b0000110000000000),
					( true,  true, false) => TileSet::new(0b0000001100000000),
					(false, false,  true) => TileSet::new(0b0000000011000000),
					( true, false,  true) => TileSet::new(0b0000000000110000),
					(false,  true,  true) => TileSet::new(0b0000000000001100),
					( true,  true,  true) => TileSet::new(0b0000000000000011),
				};
			acc | compatible_tiles
		});

	let total = bottom_right | bottom_left | top_right | top_left;
	(board.current_player.tiles & total).size() as i8 - (board.opponent.tiles & total).size() as i8
}

fn half_tiles(board: &Board) -> i8 {
	let x = board.pieces_mask;
	let horizontal_tops =
		MaskIterator(x & x >> 1 & !(x >> 7) & !(x >> 8)).fold(TileSet::empty(), |acc, half| {
			let left_black_presence = board.black_mask & half != 0;
			let right_black_presence = board.black_mask & (half << 1) != 0;
			acc | match (left_black_presence, right_black_presence) {
				(false, false) => TileSet::new(0b1000100010001000),
				(true, false) => TileSet::new(0b01000010001000100),
				(false, true) => TileSet::new(0b00100001000100010),
				(true, true) => TileSet::new(0b00010000100010001),
			}
		});

	let horizontal_bottoms =
		MaskIterator(!x & !(x >> 1) & x >> 7 & x >> 8).fold(TileSet::empty(), |acc, half| {
			let left_black_presence = board.black_mask & (half << 7) != 0;
			let right_black_presence = board.black_mask & (half << 8) != 0;
			acc | match (left_black_presence, right_black_presence) {
				(false, false) => TileSet::new(0b1111000000000000),
				(true, false) => TileSet::new(0b0000111100000000),
				(false, true) => TileSet::new(0b0000000011110000),
				(true, true) => TileSet::new(0b0000000000001111),
			}
		});

	let vertical_lefts =
		MaskIterator(x & !(x >> 1) & x >> 7 & !(x >> 8)).fold(TileSet::empty(), |acc, half| {
			let top_black_presence = board.black_mask & half != 0;
			let bottom_black_presence = board.black_mask & (half << 7) != 0;
			acc | match (top_black_presence, bottom_black_presence) {
				(false, false) => TileSet::new(0b1010000010100000),
				(true, false) => TileSet::new(0b0101000001010000),
				(false, true) => TileSet::new(0b0000101000001010),
				(true, true) => TileSet::new(0b0000010100000101),
			}
		});

	let vertical_rights =
		MaskIterator(!(x) & x >> 1 & !(x >> 7) & x >> 8).fold(TileSet::empty(), |acc, half| {
			let top_black_presence = board.black_mask & (half << 1) != 0;
			let bottom_black_presence = board.black_mask & (half << 8) != 0;
			acc | match (top_black_presence, bottom_black_presence) {
				(false, false) => TileSet::new(0b1100110000000000),
				(true, false) => TileSet::new(0b0011001100000000),
				(false, true) => TileSet::new(0b0000000011001100),
				(true, true) => TileSet::new(0b0000000000110011),
			}
		});

	let total = horizontal_tops | horizontal_bottoms | vertical_lefts | vertical_rights;
	(board.current_player.tiles & total).size() as i8 - (board.opponent.tiles & total).size() as i8
}

struct MaskIterator(u64);

impl Iterator for MaskIterator {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		if self.0 == 0 {
			return None;
		}
		let prev = self.0;
		self.0 &= self.0 - 1;
		let top_left = prev - self.0;
		Some(top_left)
	}
}
