use crate::color::Color;
use crate::tileset::TileSet;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Player {
	tiles: TileSet,
	// Used in heuristics.
	pub favorite_color: Color
}


impl Player {
	pub fn default_set() -> (Player, Player) {
		(
			Player::new([0, 1, 2, 3, 4, 5, 6, 7]),
			Player::new([15, 14, 13, 12, 11, 10, 9, 8])
		)
	}

	pub fn random_set() -> (Player, Player) {
		use rand::seq::IteratorRandom;

		let mut rng = rand::thread_rng();
		let mut buf = [0u8; 8];

		(0..16).into_iter().choose_multiple_fill(&mut rng, &mut buf);
		Player::from_current_tiles(buf)
	}

	/**
	 * Given a valid list of tiles (8 different tiles, unchecked there)
	 * return two players: the first one is the current player, the
	 * second one is its opponent.
	 */
	pub fn from_current_tiles(tiles: [u8; 8]) -> (Player, Player) {
		let ts1 = TileSet::from(tiles);

		let mut opponent_tiles = [0; 8];
		let mut i = 0;
		for tile in 0..16 {
			if ts1.has(tile) { continue }

			opponent_tiles[i] = tile;
			i += 1;
		}
		(Player::new(ts1), Player::new(opponent_tiles))
	}

	fn new(tiles: impl Into<TileSet>) -> Player {
		let ts = tiles.into();
		Player {
			tiles: ts,
			favorite_color: ts.most_present_color()
		}
	}

	pub fn has_tile(&self, tile: u8) -> bool {
		self.tiles.has(tile)
	}

	pub fn fen_part(&self) -> String {
		self.tiles
			.into_iter()
			.map(|tile|format!("{:x}", tile))
			.collect::<String>()
	}

	pub fn for_console(&self, filled_tiles: &Vec<u8>) -> String {
		let mut str = String::with_capacity(209);
		for tile in self.tiles {
			str.push_str(&format!("  {: >2}  ", tile));
		};
		str.push('\n');
		for y in 0..2 {
			let mut first_tile = true;

			for tile in self.tiles {
				if first_tile {
					first_tile = false;
				} else {
					str.push(' ')
				}
				if filled_tiles.into_iter().any(|t| *t == tile) {
					str.push_str("\x1b[44m");
				} else {
					str.push_str("\x1b[47m");
				}
				for x in 0..2 {
					// Tiles are counted horizontally starting at top left.
					let position = 1 << x << 2 * y;

					if tile & position != 0 {
						str.push_str(format!(" {}●", Color::Black).as_str());
					} else {
						str.push_str(format!(" {}●", Color::White).as_str());
					}
				}
				str.push_str(" \x1b[0m");
			}
			str.push('\n');
		}
		str
	}
}

impl std::fmt::Display for Player {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.fen_part())
	}
}

#[test]
#[ignore = "purely visual test (cargo test -- --nocapture test_show)"]
fn test_show_players() {
	let (p1, p2) = Player::default_set();
	println!("{}\n{}", p1, p2); /* use cargo test -- --nocapture */
}

#[test]
fn test_fen_part() {
	let player = Player::new([15, 14, 13, 12, 11, 10, 9, 8]);
	assert_eq!(player.fen_part(), "89abcdef");
}
