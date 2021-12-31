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
		let ts1 = TileSet::from([0, 1, 2, 3, 4, 5, 6, 7]);
		let ts2 = TileSet::from([15, 14, 13, 12, 11, 10, 9, 8]);
		(
			Player { tiles: ts1, favorite_color: ts1.most_present_color() },
			Player { tiles: ts2, favorite_color: ts2.most_present_color() }
		)
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
		let ts2 = TileSet::from(opponent_tiles);
		(
			Player { tiles: ts1, favorite_color: ts1.most_present_color() },
			Player { tiles: ts2, favorite_color: ts2.most_present_color() }
		)
	}

	pub fn has_tile(&self, tile: u8) -> bool {
		self.tiles.has(tile)
	}

	pub fn fen_part(&self) -> String {
		self.tiles
			.into_iter()
			.map(|tile|tile.to_string())
			.collect::<String>()
	}

	pub fn fmt_with_filled_tiles(&self, f: &mut std::fmt::Formatter<'_>, tiles: &Vec<u8>) -> std::fmt::Result {
		for tile in self.tiles {
			write!(f, "  {: >2}  ", tile)?;
		}
		writeln!(f, "")?;
		for y in 0..2 {
			let mut line = String::new();
			let mut first_tile = true;

			for tile in self.tiles {
				if first_tile {
					first_tile = false;
				} else {
					line.push_str(" ")
				}
				if tiles.into_iter().any(|t| *t == tile) {
					line.push_str("\x1b[44m");
				} else {
					line.push_str("\x1b[47m");
				}
				for x in 0..2 {
					// Tiles are counted horizontally starting at top left.
					let position = 1 << x << 2 * y;

					if tile & position != 0 {
						line.push_str(" \x1b[30m●");
					} else {
						line.push_str(" \x1b[31m●");
					}
				}
				line.push_str(" \x1b[0m");
			}
			writeln!(f, "{}", line)?;
		}
		Ok(())
	}
}

impl std::fmt::Display for Player {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.fmt_with_filled_tiles(f, &Vec::new())
	}
}

#[test]
#[ignore = "purely visual test (cargo test -- --nocapture test_show)"]
fn test_show_players() {
	let (p1, p2) = Player::default_set();
	println!("{}\n{}", p1, p2); /* use cargo test -- --nocapture */
}
