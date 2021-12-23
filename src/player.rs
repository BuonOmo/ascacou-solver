#[derive(PartialEq, Eq, Clone, Copy)]

struct TileSet {
	values: [bool; 16]
}

impl TileSet {
	fn has(&self, val: u8) -> bool {
		self.values[val as usize]
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
	tiles: TileSet
}


impl Player {
	pub fn news() -> (Player, Player) {
		(
			Player { tiles: TileSet::from([0, 1, 2, 3, 4, 5, 6, 7]) },
			Player { tiles: TileSet::from([15, 14, 13, 12, 11, 10, 9, 8]) }
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
