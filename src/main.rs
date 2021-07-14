use std::fmt;

fn main() {
    let res = find_options(&vec![1,1,1,1]);
    println!("{:?}",res.len());
    
}

const BLACK: &str = "x";
const WHITE: &str = " ";
const UNKNOWN: &str = "?";

const SIZE: usize = 80;

type Header = Vec<usize>;

type SideHeader = [Header; SIZE];

#[derive(Debug)]
struct Board {
    row_nums: SideHeader,
    col_nums: SideHeader,
}

// impl Default for Board {
//     fn default() -> Self {
//         Self {
//             row_nums: [
//                 vec![7, 2, 2, 7],
//                 vec![1, 1, 1, 2, 1, 1],
//                 vec![1, 3, 1, 3, 1, 1, 3, 1],
//                 vec![1, 3, 1, 2, 1, 1, 3, 1],
//                 vec![1, 3, 1, 2, 1, 3, 1],
//                 vec![1, 1, 2, 2, 1, 1],
//                 vec![7, 1, 1, 1, 7],
//                 vec![2],
//                 vec![2, 3, 2, 1, 4],
//                 vec![1, 1, 3, 3, 2, 1],
//                 vec![3, 1, 3, 2, 2],
//                 vec![1, 1, 1, 3, 1, 1],
//                 vec![1, 5, 1, 1, 1, 1],
//                 vec![1, 1, 1, 1, 3, 1],
//                 vec![7, 1, 1],
//                 vec![1, 1, 1, 1, 1, 1, 1, 1],
//                 vec![1, 3, 1, 1, 1, 2, 2],
//                 vec![1, 3, 1, 2, 1, 2, 1, 1],
//                 vec![1, 3, 1, 1, 1, 2],
//                 vec![1, 1, 2, 1, 1],
//                 vec![7, 1, 3, 1],
//             ],
//             col_nums: [
//                 vec![7, 1, 2, 7],
//                 vec![1, 1, 1, 1, 1, 1],
//                 vec![1, 3, 1, 1, 1, 3, 1],
//                 vec![1, 3, 1, 1, 1, 1, 3, 1],
//                 vec![1, 3, 1, 1, 1, 1, 3, 1],
//                 vec![1, 1, 2, 1, 1],
//                 vec![7, 1, 1, 1, 7],
//                 vec![4],
//                 vec![4, 2, 2, 2, 2, 2],
//                 vec![1, 2, 1, 1, 1, 2, 3],
//                 vec![1, 2, 2, 2],
//                 vec![2, 3, 1, 1, 1, 1, 1],
//                 vec![3, 3, 2, 3, 1, 1],
//                 vec![1, 1, 3, 2],
//                 vec![7, 1, 1],
//                 vec![1, 1, 1, 1, 1, 1, 1],
//                 vec![1, 3, 1, 3, 2, 3],
//                 vec![1, 3, 1, 2, 2, 1, 1],
//                 vec![1, 3, 1, 1, 1, 1, 1],
//                 vec![1, 1, 5, 3],
//                 vec![7, 1, 1, 2, 1],
//             ],
//         }
//     }
// }

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

fn find_options(header: &Header) ->Vec<String>{
    let mut x :Vec<String> = Vec::<String>::new();
    rec_find_options(header, String::from(""), &mut x);
    x
}

fn rec_find_options(header: &Header, so_far: String, result: &mut Vec<String>) {
    if header.len() == 0 {
        let len =so_far.len();
        let s = [so_far,WHITE.repeat(SIZE-len)].join("");
        result.push(s)
    } else {
        let mut remaining = header.to_vec();
        remaining.remove(0);
        let max_possible = SIZE - min_size(&remaining) - so_far.len();
        let mut range = max_possible - header[0];
        if remaining.len()==0{
            range = range+1;
        }
        for i in 0..range{
            let end:String= if remaining.len()==0 {String::from("")} else {String::from(" ")};
            let string = [so_far.to_owned(),WHITE.repeat(i),BLACK.repeat(header[0]),end].join("");
            rec_find_options(&remaining, string, result);
        }
    }
}
