pub(crate) mod util;

use std::time::{Duration, Instant};
use std::{sync::mpsc, thread};

use crate::util::FILES;

fn run_one<S>(fen: impl AsRef<str>, solver: S) -> Result<(Duration, u128), &'static str>
where
	S: Fn(&ascacou::Board) -> u128 + Send + 'static,
{
	let board = ascacou::Board::from_fen(fen.as_ref()).map_err(|_| "Could not parse FEN")?;
	let (sender, receiver) = mpsc::channel();
	let runner = thread::spawn(move || {
		let start = Instant::now();
		let positions = solver(&board);
		sender.send((start.elapsed(), positions)).ok();
	});
	thread::sleep(Duration::from_secs(5));
	let (duration, positions) = receiver.try_recv().map_err(|_| "No result in time")?;
	runner.join().map_err(|_| "runner panicked")?;

	Ok((duration, positions))
}

fn run_group<S>(
	file: impl AsRef<str>,
	solver: S,
	max_iter: usize,
) -> Result<(Duration, u128), &'static str>
where
	S: Fn(&ascacou::Board) -> u128 + Send + 'static + Copy,
{
	let content =
		std::fs::read_to_string(file.as_ref()).map_err(|_| "Could not read benchmark file")?;
	let mut total_duration = Duration::ZERO;
	let mut total_positions = 0u128;
	let lines: Vec<&str> = content.lines().collect();
	let count = lines.len();
	let count = if count > max_iter { max_iter } else { count };
	let mut i = 0;
	for line in lines {
		i += 1;
		if i > count {
			break;
		}
		let (duration, positions) = run_one(line, solver)?;
		total_duration += duration;
		total_positions += positions;
	}
	let avg_duration = total_duration / count as u32;
	let avg_positions = total_positions / count as u128;

	Ok((avg_duration, avg_positions))
}

fn full(board: &ascacou::Board) -> u128 {
	minicou::solve(board, Some(51)).2
}

fn shallow(board: &ascacou::Board) -> u128 {
	minicou::solve(board, Some(8)).2
}

fn main() -> Result<(), &'static str> {
	let dir = std::path::Path::new("benchmarks/data");
	if !dir.exists() {
		return Err("benchmarks/data directory does not exist");
	}
	std::env::set_current_dir(dir).map_err(|_| "Could not change directory")?;

	let shallow: fn(&ascacou::Board) -> u128 = shallow;
	let full: fn(&ascacou::Board) -> u128 = full;
	let solvers = [("full", full), ("shallow", shallow)];

	println!("test\tavg time\tavg pos\tpos/ms");
	for file in FILES {
		for (name, solver) in solvers {
			match run_group(file, solver, 2) {
				Ok((avg_duration, avg_positions)) => {
					println!(
						"{}-{}\t{:.2?}\t{}\t{}",
						file,
						name,
						avg_duration,
						avg_positions,
						avg_positions as f64 / avg_duration.as_secs_f64() / 1000.0
					);
				}
				Err(_) => {
					println!("{}-{}\tn/a\tn/a\tn/a", file, name)
				}
			}
		}
	}

	Ok(())
}
