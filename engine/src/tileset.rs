use crate::color::Color;

const BLACK_COLOR_PRESENCE: [u8; 16] = [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4];

// TODO(perf): we should be able to borrow players and tilesets
//   since those are never muted.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct TileSet {
	values: [bool; 16],
}

impl TileSet {
	pub fn has(&self, val: u8) -> bool {
		self.values[val as usize]
	}

	pub const fn most_present_color(&self) -> Color {
		let mut count = 0;
		let mut i = 0;

		while i < 16 {
			if self.values[i] {
				count += BLACK_COLOR_PRESENCE[i];
			}
			i += 1;
		}

		if count >= 16 {
			Color::Black
		} else {
			Color::White
		}
	}
}

impl std::iter::IntoIterator for TileSet {
	type Item = u8;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		let mut rv = Vec::with_capacity(16);
		let mut i = 0u8;
		for has_it in self.values {
			if has_it {
				rv.push(i)
			}
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
		tiles.into_iter().collect()
	}
}

impl From<[u8; 8]> for TileSet {
	fn from(tiles: [u8; 8]) -> TileSet {
		tiles.iter().map(|e| *e).collect()
	}
}

impl From<std::collections::LinkedList<u8>> for TileSet {
	fn from(tiles: std::collections::LinkedList<u8>) -> TileSet {
		tiles.into_iter().collect()
	}
}

impl std::iter::FromIterator<u8> for TileSet {
	fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> TileSet {
		let mut values = [false; 16];
		for tile in iter {
			values[tile as usize] = true;
		}
		TileSet { values }
	}
}
