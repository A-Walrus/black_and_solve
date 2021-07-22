use serde::{Deserialize, Serialize};
use serde_json;
use std::{fmt, fs, ops, time::SystemTime};

fn main() {
	let b = Board::default();
	let start = SystemTime::now();
	for _ in 0..1000 {
		b.solve();
	}
	println!("{:?}", start.elapsed());
}

const SIZE: usize = 20;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
	Black,
	White,
	Unknown,
}

impl fmt::Debug for Tile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let tile = match self {
			Tile::Black => '■',
			Tile::White => ' ',
			Tile::Unknown => '?',
		};
		write!(f, "{}", tile)
	}
}

impl ops::Add<Tile> for &Tile {
	type Output = Tile;
	fn add(self, rhs: Tile) -> Tile {
		match self {
			Tile::Unknown => Tile::Unknown,
			_ => {
				if *self == rhs {
					*self
				} else {
					Tile::Unknown
				}
			}
		}
	}
}

type Line = [Tile; SIZE];

type Header = Vec<usize>;

type SideHeader = [Header; SIZE];

type Options = Vec<Line>;

#[derive(Debug, Serialize, Deserialize)]
struct Board {
	row: SideHeader,
	column: SideHeader,
}

impl Board {
	fn solve(&self) {
		let mut row_options: Vec<Options> = self.row.iter().map(find_options).collect();
		let mut col_options: Vec<Options> = self.column.iter().map(find_options).collect();
		let mut row_summaries: Vec<Line> = row_options.iter().map(|x| summarize(&x)).collect();
		let mut col_summaries: Vec<Line> = col_options.iter().map(|x| summarize(&x)).collect();

		while !done(&row_summaries) {
			filter(&row_summaries, &mut col_options);
			filter(&col_summaries, &mut row_options);
			row_summaries = row_options.iter().map(|x| summarize(&x)).collect();
			col_summaries = col_options.iter().map(|x| summarize(&x)).collect();
		}

		// for row in row_summaries.iter() {
		// 	for square in row {
		// 		print!("{:?} ", square);
		// 	}
		// 	println!();
		// }
		// println!();
	}
}

fn filter(from: &Vec<Line>, to: &mut Vec<Options>) {
	for (l, line) in from.iter().enumerate() {
		for (t, tile) in line.iter().enumerate() {
			match tile {
				Tile::Unknown => (),
				_ => {
					to[t].retain(|option_col| option_col[l] == *tile);
				}
			};
		}
	}
}

fn done(lines: &Vec<Line>) -> bool {
	for line in lines.iter() {
		for square in line.iter() {
			match square {
				Tile::Unknown => return false,
				_ => (),
			}
		}
	}
	true
}

fn min_size(header: &[usize]) -> usize {
	if header.len() > 0 {
		let mut sum = 0;
		for val in header.iter() {
			sum += val;
		}
		sum + header.len()
	} else {
		0
	}
}

fn find_options(header: &Header) -> Options {
	let mut res = Vec::<Line>::new();
	rec_find_options(&header[..], [Tile::White; SIZE], 0, &mut res);
	res
}

fn rec_find_options(header: &[usize], line: Line, filled: usize, result: &mut Options) {
	if header.len() > 0 {
		let remaining = &header[1..];
		for i in filled..SIZE - min_size(remaining) - header[0] + 1 {
			let mut copy = line;
			for j in i..i + header[0] {
				copy[j] = Tile::Black;
			}
			rec_find_options(&remaining, copy, i + header[0] + 1, result);
		}
	} else {
		result.push(line);
	}
}

fn summarize(options: &Options) -> Line {
	let mut res = options[0];
	for line in options.iter() {
		for (i, tile) in line.iter().enumerate() {
			res[i] = tile + res[i];
		}
	}
	res
}

impl Default for Board {
	fn default() -> Self {
		let data = fs::read_to_string(r"D:\Users\Guy\rust\Projects\black_and_solve\puzzles\nonogram-hints.json")
			.expect("Unable to read file");
		serde_json::from_str(&data).expect("bad json")
	}
}
