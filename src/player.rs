use crate::color::Color;
use crate::tileset::TileSet;

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

	pub fn fmt_with_filled_tiles(&self, f: &mut std::fmt::Formatter<'_>, tiles: &Vec<u8>) -> std::fmt::Result {
		for tile in self.tiles {
			write!(f, "  {: >2}  ", tile);
		}
		writeln!(f, "");
		for x in 0..2 {
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

impl std::fmt::Display for Player {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.fmt_with_filled_tiles(f, &Vec::new())
	}
}

#[test]
#[ignore = "reversed for now, but who cares"]
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
