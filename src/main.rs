use std::{time::{SystemTime},ops,fmt};

fn main() {

    let start = SystemTime::now();
    let b = Board::default();
    b.solve();
    println!("{:?}",start.elapsed());
}

const SIZE: usize = 21;

#[derive(Clone, Copy,Eq,PartialEq)]
enum Tile {
    Black,
    White,
    Unknown,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tile = match self{
            Tile::Black=>{'■'},
            Tile::White=>{' '},
            Tile::Unknown=>{'?'},
        };
        write!(f, "{}",tile)
    }
}

impl ops::Add<Tile> for &Tile{
    type Output=Tile;
    fn add(self,rhs:Tile)->Tile{
        match self{
            Tile::Unknown=>{Tile::Unknown},
            _=>{if *self==rhs {*self} else {Tile::Unknown}}
        }
    }
}

type Line = [Tile; SIZE];

type Header = Vec<usize>;

type SideHeader = [Header; SIZE];

type Options = Vec<Line>;

#[derive(Debug)]
struct Board {
    row_nums: SideHeader,
    col_nums: SideHeader,
}




impl Board{
    fn solve(&self){
        let mut row_options :Vec<Options>= self.row_nums.iter().map(find_options).collect();
        let mut col_options :Vec<Options>= self.col_nums.iter().map(find_options).collect();
        let mut row_summaries :Options = row_options.iter().map(|x|summarize(&x)).collect();
        let mut col_summaries :Options = col_options.iter().map(|x|summarize(&x)).collect();
        
        while !done(&row_summaries) {
            filter(&mut row_summaries,&mut col_options);
            filter(&mut col_summaries,&mut row_options);
            row_summaries = row_options.iter().map(|x|summarize(&x)).collect();
            col_summaries = col_options.iter().map(|x|summarize(&x)).collect();
        }


        // for row in row_summaries.iter(){
        //     println!("{:?}",row);
        // }
        // println!();

    }
}

fn filter(from:&mut Options,to:&mut Vec<Options>){
    for (r,row) in from.iter().enumerate(){
        for (t,tile) in row.iter().enumerate(){
            match tile{
                Tile::Unknown=>(),
                _=>{
                    to[t] = to[t].drain(..).filter(|option_col|option_col[r]==*tile).collect();
                }
            };
        }
    }
}


fn done(lines:&Options)->bool{
    for line in lines.iter(){
        for square in line.iter(){
            match square{
                Tile::Unknown=>{return false},
                _=>()
            }
        }
    }
    true
}

fn min_size(header: &Header) -> usize {
    if header.len() > 0 {
        let mut sum = 0;
        for val in header.iter() {
            sum += val;
        }
        sum + header.len() - 1
    } else {
        0
    }
}

fn find_options(header: &Header) -> Options {
    let mut res = Vec::<Line>::new();
    rec_find_options(header, [Tile::White; SIZE], 0, &mut res);
    res
}

fn rec_find_options(header: &Header, line: Line, filled: usize, result: &mut Options) {
    if header.len() > 0 {
        let mut remaining = header.to_vec();
        remaining.remove(0);
        let available = SIZE - filled - min_size(&remaining);
        let space = if remaining.len() == 0 {
            header[0]
        } else {
            header[0] + 1
        };
        let range = available - space;
        for i in filled..filled + range + 1 {
            let mut copy = line;
            for j in filled..i {
                copy[j] = Tile::White;
            }
            for j in i..i + header[0] {
                copy[j] = Tile::Black;
            }
            rec_find_options(&remaining, copy, i + space, result);
        }
    } else {
        result.push(line);
    }
}

fn summarize(options:&Options)->Line{
    let mut res = options[0];
    for line in options.iter(){
        for (i,tile) in line.iter().enumerate(){
            res[i]=tile+res[i];
        }
    }
    res
}


impl Default for Board {
    fn default() -> Self {
        Self {
            row_nums: [
                vec![7, 2, 2, 7],
                vec![1, 1, 1, 2, 1, 1],
                vec![1, 3, 1, 3, 1, 1, 3, 1],
                vec![1, 3, 1, 2, 1, 1, 3, 1],
                vec![1, 3, 1, 2, 1, 3, 1],
                vec![1, 1, 2, 2, 1, 1],
                vec![7, 1, 1, 1, 7],
                vec![2],
                vec![2, 3, 2, 1, 4],
                vec![1, 1, 3, 3, 2, 1],
                vec![3, 1, 3, 2, 2],
                vec![1, 1, 1, 3, 1, 1],
                vec![1, 5, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 3, 1],
                vec![7, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 3, 1, 1, 1, 2, 2],
                vec![1, 3, 1, 2, 1, 2, 1, 1],
                vec![1, 3, 1, 1, 1, 2],
                vec![1, 1, 2, 1, 1],
                vec![7, 1, 3, 1],
            ],
            col_nums: [
                vec![7, 1, 2, 7],
                vec![1, 1, 1, 1, 1, 1],
                vec![1, 3, 1, 1, 1, 3, 1],
                vec![1, 3, 1, 1, 1, 1, 3, 1],
                vec![1, 3, 1, 1, 1, 1, 3, 1],
                vec![1, 1, 2, 1, 1],
                vec![7, 1, 1, 1, 7],
                vec![4],
                vec![4, 2, 2, 2, 2, 2],
                vec![1, 2, 1, 1, 1, 2, 3],
                vec![1, 2, 2, 2],
                vec![2, 3, 1, 1, 1, 1, 1],
                vec![3, 3, 2, 3, 1, 1],
                vec![1, 1, 3, 2],
                vec![7, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1],
                vec![1, 3, 1, 3, 2, 3],
                vec![1, 3, 1, 2, 2, 1, 1],
                vec![1, 3, 1, 1, 1, 1, 1],
                vec![1, 1, 5, 3],
                vec![7, 1, 1, 2, 1],
            ],
        }
    }
}
