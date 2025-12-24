use crate::color::Color;

const BLACK_COLOR_PRESENCE: [u8; 16] = [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4];

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct TileSet(u16);

impl TileSet {
	pub const fn empty() -> Self {
		TileSet::new(0)
	}

	pub const fn new(values: u16) -> Self {
		TileSet(values)
	}

	pub const fn has(&self, val: u8) -> bool {
		debug_assert!(val < 16);
		self.0 & (1 << val) != 0
	}

	pub const fn try_add(&mut self, val: u8) -> Option<TileSet> {
		debug_assert!(val < 16);
		let tile_mask = 1 << val;
		// We could use `has` here, but this avoids one
		// extra bit shift operation.
		match self.0 & tile_mask {
			0 => Some(TileSet::new(self.0 | tile_mask)),
			_ => None,
		}
	}

	pub const fn try_union(&self, other: &TileSet) -> Option<TileSet> {
		match self.0 & other.0 {
			0 => Some(TileSet::new(self.0 | other.0)),
			_ => None,
		}
	}

	pub const fn size(&self) -> u8 {
		self.0.count_ones() as u8
	}

	pub const fn is_full(&self) -> bool {
		self.0 == 0xFFFF
	}

	pub const fn most_present_color(&self) -> Color {
		if self.count_blacks() >= 16 {
			Color::Black
		} else {
			Color::White
		}
	}

	const fn count_blacks(&self) -> u8 {
		let mut count = 0;
		let mut values = self.0;

		while values != 0 {
			let i = values.trailing_zeros() as usize;
			values ^= 1 << i;
			count += BLACK_COLOR_PRESENCE[i];
		}

		count
	}
}

impl std::ops::Not for TileSet {
	type Output = TileSet;

	fn not(self) -> TileSet {
		// NOTE: this only works because a tileset
		// has exactly 16 tiles, hence every bit
		// matters. Otherwise this would overflow
		// in unknown territory.
		TileSet::new(!self.0)
	}
}

impl std::ops::BitAnd for TileSet {
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self::Output {
		Self(self.0 & rhs.0)
	}
}

impl std::ops::BitOr for TileSet {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output {
		Self(self.0 | rhs.0)
	}
}

impl std::fmt::Debug for TileSet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("TileSet({:0>16b})", self.0))
	}
}

impl Iterator for TileSet {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		if self.0 == 0 {
			return None;
		}
		let i = self.0.trailing_zeros() as u8;
		self.0 ^= 1 << i;
		Some(i)
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
		let mut values = 0u16;
		for tile in iter {
			values |= 1 << tile;
		}
		TileSet::new(values)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tileset_iterator() {
		for tiles in vec![
			vec![0, 3, 5, 7, 8, 9, 10, 15],
			vec![1, 2, 4, 6, 8, 9, 10, 11],
		] {
			let tileset: TileSet = tiles.clone().into();
			let collected_tiles: Vec<u8> = tileset.into_iter().collect();
			assert_eq!(tiles, collected_tiles);
		}
	}

	#[test]
	fn test_tileset_has() {
		let tileset: TileSet = vec![0, 3, 5, 7, 8, 9, 10, 15].into();
		assert!(tileset.has(0));
		assert!(tileset.has(3));
		assert!(!tileset.has(4));
		assert!(tileset.has(9));
		assert!(!tileset.has(14));
		assert!(tileset.has(15));
		let tileset: TileSet = vec![1, 2, 4, 6, 8, 9, 10, 11].into();
		assert!(!tileset.has(0));
		assert!(!tileset.has(15));
	}

	#[test]
	#[should_panic(expected = "val < 16")]
	fn test_tileset_has_invalid_value() {
		let tileset: TileSet = vec![0, 3, 5, 7, 8, 9, 10, 15].into();
		tileset.has(16);
	}

	#[test]
	fn test_count_blacks() {
		let mostly_white: TileSet = vec![0, 1, 2, 3, 4, 5, 6, 7].into();
		let mostly_black: TileSet = vec![8, 9, 10, 11, 12, 13, 14, 15].into();
		let same_same: TileSet = vec![4, 5, 6, 7, 8, 9, 10, 11].into();

		assert_eq!(mostly_white.count_blacks(), 12);
		assert_eq!(mostly_black.count_blacks(), 20);
		assert_eq!(same_same.count_blacks(), 16);
	}
}
