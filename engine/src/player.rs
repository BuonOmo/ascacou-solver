use crate::color::Color;
use crate::tileset::TileSet;
use rand::Rng;
use rand::seq::IteratorRandom;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Player {
	tiles: TileSet,
	// Used in heuristics.
	pub favorite_color: Color,
}

impl Player {
	pub fn default_set() -> (Player, Player) {
		(
			Player::new([0, 1, 2, 3, 4, 5, 6, 7]),
			Player::new([15, 14, 13, 12, 11, 10, 9, 8]),
		)
	}

	pub fn random_set<R: Rng>(rng: &mut R) -> (Player, Player) {
		(0..16)
			.choose_multiple(rng, 8)
			.iter()
			.cloned()
			.collect::<TileSet>()
			.into()
	}

	fn new(tiles: impl Into<TileSet>) -> Player {
		let ts = tiles.into();
		Player {
			tiles: ts,
			favorite_color: ts.most_present_color(),
		}
	}

	pub fn has_tile(&self, tile: u8) -> bool {
		self.tiles.has(tile)
	}

	pub fn fen_part(&self) -> String {
		self.tiles
			.into_iter()
			.map(|tile| format!("{:x}", tile))
			.collect::<String>()
	}

	pub fn for_console(&self, played_tiles: &TileSet) -> String {
		let mut str = String::with_capacity(209);
		for tile in self.tiles {
			str.push_str(&format!("  {: >2}  ", tile));
		}
		str.push('\n');
		for y in 0..2 {
			let mut first_tile = true;

			for tile in self.tiles {
				if first_tile {
					first_tile = false;
				} else {
					str.push(' ')
				}
				if played_tiles.has(tile) {
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

/// Given a valid list of tiles (8 different tiles, unchecked here)
/// return two players: the first one is the current player, the
/// second one is its opponent.
impl From<TileSet> for (Player, Player) {
	fn from(tileset: TileSet) -> (Player, Player) {
		(Player::new(tileset), Player::new(!tileset))
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
