pub(crate) mod util;

use std::time::{Duration, Instant};

use crate::util::{FILES, MAX_DEPTHS};

fn run_one(fen: impl AsRef<str>, depth: u8) -> Result<(Duration, u128), &'static str> {
	let board = ascacou::Board::from_fen(fen.as_ref()).map_err(|_| "Could not parse FEN")?;
	let time = Instant::now();
	let (_, _, positions) = minicou::solve(&board, Some(depth));
	let duration = time.elapsed();
	Ok((duration, positions))
}

fn run_group(
	file: impl AsRef<str>,
	max_depth: u8,
	max_iter: usize,
) -> Result<(u8, Duration, u128), &'static str> {
	let content =
		std::fs::read_to_string(file.as_ref()).map_err(|_| "Could not read benchmark file")?;
	let mut total_duration = Duration::ZERO;
	let mut total_positions = 0u128;
	let lines: Vec<&str> = content.lines().collect();
	let count = lines.len();
	let count = if count > max_iter { max_iter } else { count };
	let depth = find_depth(lines[0], max_depth)?;
	let mut i = 0;
	for line in lines {
		i += 1;
		if i > count {
			break;
		}
		let (duration, positions) = run_one(line, depth)?;
		total_duration += duration;
		total_positions += positions;
	}
	let avg_duration = total_duration / count as u32;
	let avg_positions = total_positions / count as u128;

	Ok((depth, avg_duration, avg_positions))
}

fn find_depth(fen: &str, max_depth: u8) -> Result<u8, &'static str> {
	let mut max = 1;
	for i in 2..=max_depth {
		let (duration, _) = run_one(fen, i)?;
		if duration.as_secs_f64() > 1.0 {
			break;
		} else {
			max = i;
		}
	}
	Ok(max)
}

fn pos_per_ms(positions: u128, duration: Duration) -> String {
	let freq = positions as f64 / duration.as_secs_f64();
	let (freq, unit) = if freq > 999_999.0 {
		(freq / 1_000_000.0, "M")
	} else if freq > 999.0 {
		(freq / 1_000.0, "K")
	} else {
		(freq, "")
	};
	format!("{:.2}{}", freq, unit)
}

fn main() -> Result<(), &'static str> {
	let dir = std::path::Path::new("benchmarks/data");
	if !dir.exists() {
		return Err("benchmarks/data directory does not exist");
	}
	std::env::set_current_dir(dir).map_err(|_| "Could not change directory")?;

	println!("sample\tdepth\tavg time\tavg pos\tpos/ms");
	for (&file, &max_depth) in std::iter::zip(FILES, MAX_DEPTHS) {
		match run_group(file, max_depth, 10) {
			Ok((depth, avg_duration, avg_positions)) => {
				println!(
					"{}\t{}\t{:.2?}\t{}\t{}",
					file,
					if depth < max_depth {
						depth.to_string()
					} else {
						"max".to_string()
					},
					avg_duration,
					avg_positions,
					pos_per_ms(avg_positions, avg_duration)
				);
			}
			Err(_) => {
				println!("{}\t0\tn/a\tn/a\tn/a", file)
			}
		}
	}

	Ok(())
}
