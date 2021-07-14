use std::fmt;



fn main() {
    let nonogram = Board::default();
}

const SIZE:usize=21;

type Header = Vec<usize>;

type SideHeader = [Header;SIZE];

#[derive(Copy,Clone)]
enum Square {
    Black,
    White
}

type Tile = Option<Square>;

type Line = [Tile;SIZE];

struct Board{
    row_nums:SideHeader,
    col_nums:SideHeader,
    // tiles[row][column]
    tiles:[[Option<Square>;SIZE];SIZE]
}

impl Default for Board{
    fn default()->Self{
        Self{
            row_nums:[  
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
                vec![7, 1, 3, 1]],
            col_nums:[
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
                vec![7, 1, 1, 2, 1],],
            tiles:Default::default()
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn fmt_tile(tile:&Tile)->&str{
            match tile{
                Some(square)=>{
                    match square{
                        Square::Black=>"■",
                        Square::White=>" ",
                    }
                },
                None=>{"."}
            }
        }


        for row in &self.tiles {
            for tile in row {
                write!(f, "{} ", fmt_tile(tile))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

enum Direction{
    Row,
    Column,
}

impl Board{
    fn get_line(& mut self,index:usize,direction:Direction)->Line{
        match direction{
            Row=>{
                self.tiles[index]
            },
            Column=>{
                let mut line = [None::<Square>;SIZE];
                for i in 0..SIZE{
                    line[i]=self.tiles[i][index];
                }
                line
            }
        }
    }
}

fn min_size(header:Header)->usize{
    if header.len()>0{
        let mut sum=0;
        for val in header.iter(){
            sum+=val;
        }
        sum+header.len()-1
    }
    else{
        0
    }
}

fn find_options(header:Header,result:&mut Vec<Line>){
    red_find_options(header, 0, [None;SIZE], result)
}

fn red_find_options(header:Header,start:usize,so_far:Line,result: &mut Vec<Line> ){
    let mut remaining = header.to_vec();
    remaining.remove(0);
    let max_possible = SIZE-min_size(remaining)-start;
    let range = max_possible-header[0];
    for i in 0..range{
        let line = so_far;
    }

}