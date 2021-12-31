use crate::color::Color;

const BLACK_COLOR_PRESENCE: [u8; 16] =
	[0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4];

// TODO(perf): we should be able to borrow players and tilesets
//   since those are never muted.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct TileSet {
	values: [bool; 16]
}

impl TileSet {
	pub fn has(&self, val: u8) -> bool {
		self.values[val as usize]
	}

	pub const fn most_present_color(&self) -> Color {
		let mut count = 0;

		if self.values[0] { count += BLACK_COLOR_PRESENCE[0] }
		if self.values[1] { count += BLACK_COLOR_PRESENCE[1] }
		if self.values[2] { count += BLACK_COLOR_PRESENCE[2] }
		if self.values[3] { count += BLACK_COLOR_PRESENCE[3] }
		if self.values[4] { count += BLACK_COLOR_PRESENCE[4] }
		if self.values[5] { count += BLACK_COLOR_PRESENCE[5] }
		if self.values[6] { count += BLACK_COLOR_PRESENCE[6] }
		if self.values[7] { count += BLACK_COLOR_PRESENCE[7] }
		if self.values[8] { count += BLACK_COLOR_PRESENCE[8] }
		if self.values[9] { count += BLACK_COLOR_PRESENCE[9] }
		if self.values[10] { count += BLACK_COLOR_PRESENCE[10] }
		if self.values[11] { count += BLACK_COLOR_PRESENCE[11] }
		if self.values[12] { count += BLACK_COLOR_PRESENCE[12] }
		if self.values[13] { count += BLACK_COLOR_PRESENCE[13] }
		if self.values[14] { count += BLACK_COLOR_PRESENCE[14] }
		if self.values[15] { count += BLACK_COLOR_PRESENCE[15] }

		if count >= 16 { Color::Black } else { Color::White }
	}
}

impl std::iter::IntoIterator for TileSet {
	type Item = u8;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		let mut rv = Vec::with_capacity(16);
		let mut i = 0u8;
		for has_it in self.values {
			if has_it { rv.push(i) }
			i += 1;
		}
		rv.into_iter()
	}
}

// TODO(refacto): There should be a way to only have one From<Iterator>
//   there
// NOTE: this is a bit unsafe, we assume tiles have a value between 0..16.
impl From<Vec<u8>> for TileSet {
	fn from(tiles: Vec<u8>) -> TileSet {
		let mut values = [false; 16];
		for tile in tiles {
			values[tile as usize] = true;
		}
		TileSet { values }
	}
}

impl From<[u8; 8]> for TileSet {
	fn from(tiles: [u8; 8]) -> TileSet {
		let mut values = [false; 16];
		for tile in tiles {
			values[tile as usize] = true;
		}
		TileSet { values }
	}
}

impl From<std::collections::LinkedList<u8>> for TileSet {
	fn from(tiles: std::collections::LinkedList<u8>) -> TileSet {
		let mut values = [false; 16];
		for tile in tiles {
			values[tile as usize] = true;
		}
		TileSet { values }
	}
}
