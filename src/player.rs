use crate::color::Color;

const BLACK_COLOR_PRESENCE: [u8; 16] =
	[0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4];

#[derive(PartialEq, Eq, Clone, Copy)]
struct TileSet {
	values: [bool; 16]
}

impl TileSet {
	fn has(&self, val: u8) -> bool {
		self.values[val as usize]
	}

	const fn most_present_color(&self) -> Color {
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

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Player {
	tiles: TileSet,
	// Used in heuristics.
	pub favorite_color: Color
}


impl Player {
	pub fn news() -> (Player, Player) {
		let ts1 = TileSet::from([0, 1, 2, 3, 4, 5, 6, 7]);
		let ts2 = TileSet::from([15, 14, 13, 12, 11, 10, 9, 8]);
		(
			Player { tiles: ts1, favorite_color: ts1.most_present_color() },
			Player { tiles: ts2, favorite_color: ts2.most_present_color() }
		)
	}

	pub fn has_tile(&self, tile: u8) -> bool {
		self.tiles.has(tile)
	}
}

impl std::fmt::Display for Player {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for tile in self.tiles {
			write!(f, "  {: >2}  ", tile);
		}
		writeln!(f, "");
		for x in 0..2 {
			let mut line = String::new();
			let mut first_tile = true;

			for tile in self.tiles {
				if first_tile {
					line.push_str("\x1b[47m"); // TODO: we could use [44m bg for completed tiles.
					first_tile = false;
				} else {
					line.push_str(" \x1b[47m");
				}
				for y in 0..2 {
					let position = 1 << x << (2*y); // TODO: 2x or 2y?

					if tile & position != 0 {
						line.push_str(" \x1b[30m●");
					} else {
						line.push_str(" \x1b[31m●");
					}
				}
				line.push_str(" \x1b[0m");
			}
			writeln!(f, "{}", line);
		}
		Ok(())
	}
}

#[test]
fn test_show_players() {
	let (p1, p2) = Player::news();
	// println!("{}\n{}", p1, p2); /* use cargo test -- --nocapture */
	assert_eq!(
		format!("{}\n{}", p1, p2),
		"\
\u{1b}[47m \u{1b}[31m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[30m● \u{1b}[0m\n\u{1b}[47m \u{1b}[31m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[31m● \u{1b}[0m\n\n\u{1b}[47m \u{1b}[30m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[31m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[31m● \u{1b}[0m
\u{1b}[47m \u{1b}[30m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[30m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[30m● \u{1b}[0m \u{1b}[47m \u{1b}[31m● \u{1b}[30m● \u{1b}[0m
"
	);
}
