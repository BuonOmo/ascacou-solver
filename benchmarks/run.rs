pub(crate) mod utils;

use std::io::Write;
use std::time::{Duration, Instant};

use crate::utils::{FILES, MAX_DEPTHS};

type SimpleResult<T> = Result<T, &'static str>;
type EmptyResult = SimpleResult<()>;

fn run_one(fen: impl AsRef<str>, depth: u8, is_partial: bool) -> SimpleResult<(Duration, u128)> {
	let board = ascacou::Board::from_fen(fen.as_ref()).map_err(|_| "Could not parse FEN")?;
	let time = Instant::now();

	let (_, _, positions) = if is_partial {
		minicou::partial_solve(&board, Some(depth))
	} else {
		minicou::solve(&board, Some(depth))
	};
	let duration = time.elapsed();
	Ok((duration, positions))
}

fn run_group(
	file: impl AsRef<str>,
	max_depth: u8,
	is_partial: bool,
	iterations: usize,
) -> SimpleResult<(u8, Duration, u128)> {
	let content =
		std::fs::read_to_string(file.as_ref()).map_err(|_| "Could not read benchmark file")?;
	let mut total_duration = Duration::ZERO;
	let mut total_positions = 0u128;
	let lines: Vec<&str> = content.lines().collect();
	let count = lines.len();
	let count = if count > iterations {
		iterations
	} else {
		count
	};
	let depth = find_depth(lines[0], max_depth, is_partial)?;
	let mut i = 0;
	for line in lines {
		i += 1;
		if i > count {
			break;
		}
		let (duration, positions) = run_one(line, depth, is_partial)?;
		total_duration += duration;
		total_positions += positions;
	}
	let avg_duration = total_duration / count as u32;
	let avg_positions = total_positions / count as u128;

	Ok((depth, avg_duration, avg_positions))
}

fn find_depth(fen: &str, max_depth: u8, is_partial: bool) -> SimpleResult<u8> {
	let mut max = 1;
	for i in 2..=max_depth {
		let (duration, _) = run_one(fen, i, is_partial)?;
		if duration.as_secs_f64() > 5.0 {
			break;
		} else {
			max = i;
		}
	}
	Ok(max)
}

fn format_pos_per_ms(positions: u128, duration: Duration) -> String {
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

fn format_depth(depth: u8, max_depth: u8, is_partial: bool) -> String {
	let mut depth = if depth < max_depth {
		depth.to_string()
	} else {
		"max".to_string()
	};
	if is_partial {
		depth.push_str(" (partial)");
	}
	depth
}

fn set_dir() -> EmptyResult {
	let dir = std::path::Path::new("benchmarks/data");
	if !dir.exists() {
		return Err("benchmarks/data directory does not exist");
	}
	std::env::set_current_dir(dir).map_err(|_| "Could not change directory")?;
	Ok(())
}

fn iterations() -> impl Iterator<Item = (bool, &'static str, u8)> + 'static {
	let length = FILES.len();
	std::iter::repeat_n(true, length)
		.chain(std::iter::repeat_n(false, length))
		.zip(std::iter::zip(FILES.iter(), MAX_DEPTHS.iter()).cycle())
		.map(|(partial, (&file, &depth))| (partial, file, depth))
}

enum Alignment {
	#[allow(dead_code)]
	Left,
	#[allow(dead_code)]
	Center,
	Right,
	Default,
}

fn generate_table(
	alignments: [Alignment; 5],
	headers: [String; 5],
	body: Vec<[String; 5]>,
) -> Vec<u8> {
	let mut buffer = Vec::new();
	let lengths: [usize; 5] = [headers.clone()]
		.iter()
		.chain(body.iter())
		.map(|[a, b, c, d, e]| [a.len(), b.len(), c.len(), d.len(), e.len()])
		.fold(
			[0; 5],
			|[acc_a, acc_b, acc_c, acc_d, acc_e], [len_a, len_b, len_c, len_d, len_e]| {
				[
					acc_a.max(len_a),
					acc_b.max(len_b),
					acc_c.max(len_c),
					acc_d.max(len_d),
					acc_e.max(len_e),
				]
			},
		);
	// format!("{:-<l$}", "")
	let underlines: Vec<String> = lengths
		.iter()
		.zip(&alignments)
		.map(|(l, a)| match a {
			Alignment::Left => format!(":{:-<width$}", "-", width = l - 1),
			Alignment::Center => format!(":{:-<width$}:", "-", width = l - 2),
			Alignment::Right => format!("{:-<width$}:", "-", width = l - 1),
			Alignment::Default => format!("{:-<width$}", "-", width = l),
		})
		.collect();

	[headers]
		.iter()
		.chain([underlines.try_into().unwrap()].iter())
		.chain(body.iter())
		.zip([lengths].iter().cycle())
		.zip([&alignments].iter().cycle())
		.for_each(|((cells, lens), alignments)| {
			for i in 0..5 {
				let (cell, len) = (&cells[i], lens[i]);
				match alignments[i] {
					Alignment::Left => write!(&mut buffer, "| {cell:<len$} ").ok(),
					Alignment::Center => write!(&mut buffer, "| {cell:^len$} ").ok(),
					Alignment::Right => write!(&mut buffer, "| {cell:>len$} ").ok(),
					Alignment::Default => write!(&mut buffer, "| {cell:len$} ").ok(),
				};
			}
			write!(&mut buffer, "|\n").ok();
		});
	buffer
}

fn write_to_readme(table: Vec<u8>) -> EmptyResult {
	let magic_string = "<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->\n";
	let readme = std::path::Path::new("../../README.md");
	if !readme.exists() {
		return Err("Could not locate README.md");
	}
	let content =
		std::fs::read_to_string(readme).map_err(|_| "Could not read content of README.md")?;

	let indices = content.match_indices(magic_string);
	match indices.collect::<Vec<(usize, &str)>>().as_slice() {
		[(start, _), (end, _)] => {
			let mut new_content = String::new();
			new_content.push_str(&content[..start + magic_string.len()]);
			new_content.push_str(&String::from_utf8_lossy(&table));
			new_content.push_str(&content[*end..]);

			let mut file = std::fs::File::create(readme)
				.map_err(|_| "Could not open README.md for writing")?;
			file.write_all(new_content.as_bytes())
				.map_err(|_| "Could not write updated content to README.md")?;
		}
		_ => {
			return Err("Could not find magic string twice in README.md");
		}
	}

	Ok(())
}

fn main() -> EmptyResult {
	set_dir()?;

	let alignments = [
		Alignment::Default,
		Alignment::Default,
		Alignment::Right,
		Alignment::Right,
		Alignment::Right,
	];
	let headers = ["sample", "depth", "avg time", "avg n pos", "pos/ms"].map(String::from);
	let body: Vec<[String; 5]> = iterations()
		.map(
			|(partial, file, max_depth)| match run_group(file, max_depth, partial, 10) {
				Ok((depth, avg_duration, avg_positions)) => [
					file.to_string(),
					format_depth(depth, max_depth, partial),
					format!("{:.2?}", avg_duration),
					avg_positions.to_string(),
					format_pos_per_ms(avg_positions, avg_duration),
				],
				Err(_) => [
					file.to_string(),
					0.to_string(),
					"n/a".to_string(),
					"n/a".to_string(),
					"n/a".to_string(),
				],
			},
		)
		.collect();

	let table = generate_table(alignments, headers, body);

	write_to_readme(table)?;

	Ok(())
}
