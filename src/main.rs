#![feature(array_methods)]

use serde::{Deserialize, Serialize};
use serde_json;
use std::{fmt, fs, ops, time::SystemTime};
use std::{sync::mpsc, thread};

fn main() {
	let b = Board::default();
	let start = SystemTime::now();
	for _ in 0..1 {
		b.solve();
	}
	println!("{:?}", start.elapsed());
}


const SIZE: usize = 50;

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

// #[derive(Debug, Serialize, Deserialize)]
struct Board {
	row: SideHeader,
	column: SideHeader,
}

fn direction_solve(header: &SideHeader, tx: mpsc::Sender<Vec<Line>>, rx: mpsc::Receiver<Vec<Line>>) {
	let mut options: Vec<Options> = header.iter().map(find_options).collect();
	loop {
		tx.send(options.iter().map(|x| summarize(&x)).collect()).unwrap_or(());
		let summary = rx.recv().unwrap();
		filter(&summary, &mut options);
		if done(&summary) {
			tx.send(options.iter().map(|x| summarize(&x)).collect()).unwrap_or(());
			break;
		}
	}
}

fn clone(head: &SideHeader) -> SideHeader {
	head.each_ref().map(|v| v.to_owned())
}

impl Board {
	fn solve(&self) {
		let (tx1, rx1) = mpsc::channel();
		let (tx2, rx2) = mpsc::channel();
		let column = clone(&self.column);
		thread::spawn(move || {
			direction_solve(&column, tx2, rx1);
		});
		direction_solve(&self.row, tx1, rx2);

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
		// let data = fs::read_to_string(r"D:\Users\Guy\rust\Projects\black_and_solve\puzzles\nonogram-hints (7).json")
		// 	.expect("Unable to read file");
		// serde_json::from_str(&data).expect("bad json")

		Board {
			row: [
				vec![6],
				vec![14],
				vec![18],
				vec![22],
				vec![16,10],

				vec![17,11],
				vec![32],
				vec![32],
				vec![33],
				vec![34],
				
				vec![35],
				vec![15,9],
				vec![12,7],
				vec![11,5],
				vec![10,5],
				
				vec![9,4],
				vec![10,3,4],
				vec![20,3],
				vec![21,9,4],
				vec![21,6,1,4],
				
				vec![16,3,1,4,5],
				vec![15,2,2,6],
				vec![13,2,8],
				vec![10,10],
				vec![11,9],
				
				vec![14,8],
				vec![14,2,9],
				vec![15,7,8],
				vec![16,3,8],
				vec![16,2,8],
				
				vec![15,3,9],
				vec![15,4,9],
				vec![25,9],
				vec![28,9],
				vec![17,3,8,4],
				
				vec![23,8],
				vec![25,8,1],
				vec![18,10,2],
				vec![16,2],
				vec![17,1,11],
				
				vec![19,16],
				vec![41],
				vec![38],
				vec![39],
				vec![1,32,1],
				
				vec![30],
				vec![30],
				vec![30,3],
				vec![30,7],
				vec![18,9],
			],
			column: [
				vec![],
				vec![1],
				vec![9],
				vec![13],
				vec![2,15,1,4],

				vec![32],
				vec![34],
				vec![34],
				vec![36],
				vec![39],
				
				vec![44],
				vec![45],
				vec![46],
				vec![46],
				vec![25,6,11],
				
				vec![24,6,11],
				vec![23,7,10],
				vec![18,2,7,9],
				vec![18,6,10],
				vec![18,4,10],
				
				vec![15,3,4,11],
				vec![10,7,3,10],
				vec![10,7,2,4,10],
				vec![9,11,6,10],
				vec![9,2,2,5,5,11],
				
				vec![9,2,2,1,11],
				vec![9,1,2,1,11],
				vec![9,1,1,1,11],
				vec![8,1,1,2,5,4],
				vec![8,1,3,11],
				
				vec![9,2,10],
				vec![10,3,10],
				vec![11,3,10],
				vec![10,3,11],
				vec![13,2,1,10],
				
				vec![15,1,2,11],
				vec![16,1,10],
				vec![20,11],
				vec![46],
				vec![44],
				
				vec![39],
				vec![26,8],
				vec![2,25],
				vec![2,1,5,14],
				vec![2,4,12],
				
				vec![1,2,2,13],
				vec![1,1,2,1,1,2],
				vec![1,1,1],
				vec![1,1],
				vec![],
				
			]
		}
		
	}
}
