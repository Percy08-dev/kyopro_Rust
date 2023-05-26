use itertools::Itertools;
use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, x:usize, 
        stage:[(usize, usize); n], 
    }

    let mut a = Vec::with_capacity(n);
    let mut tmp = stage[0].0 + stage[0].1;

    a.push(tmp + stage[0].1 * (x-1));

    for (i, j) in stage.iter().enumerate().skip(1) {
        if x + 1 - i == 0 {
            break;
        }
        tmp += j.0 + j.1;
        a.push(tmp + j.1 * (x - i - 1));
    }

    // println!("{:?}", a);
    println!("{}", a.iter().min().unwrap());
}