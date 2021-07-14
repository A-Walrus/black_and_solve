use std::time::{Duration, SystemTime};

fn main() {

    let sys_time=SystemTime::now();
    let res = find_options(&vec![1,1,1,1]);
    println!("{}",res.len());
    println!("{:?}",sys_time.elapsed());
    
}

const SIZE: usize = 100;

const WHITE:char=' ';
const BLACK:char='x';
const UNKNOWN:char='?';



type Tile=char;


type Line=[Tile;SIZE];

type Header = Vec<usize>;

type SideHeader = [Header; SIZE];

#[derive(Debug)]
struct Board {
    row_nums: SideHeader,
    col_nums: SideHeader,
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

fn find_options(header: &Header) ->Vec<Line>{
    let mut res = Vec::<Line>::new();
    rec_find_options(header, [WHITE;SIZE], 0,&mut res);
    res
}

fn rec_find_options(header: &Header, line: Line, filled:usize, result: &mut Vec<Line>) {
    if header.len()>0{
        let mut remaining = header.to_vec();
        remaining.remove(0);
        let available = SIZE-filled-min_size(&remaining);
        let space = if remaining.len()==0 {header[0]} else {header[0]+1};
        let range = available-space;
        for i in filled..filled+range+1{
            let mut copy = line;
            for j in filled..i{
                copy[j]=WHITE;
            }
            for j in i..i+header[0]{
                copy[j]=BLACK;
            }
            rec_find_options(&remaining, copy, i+space,result);
        }
    }
    else{
        result.push(line);
    }
}
