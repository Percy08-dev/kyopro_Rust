use proconio::input;
use std::cmp::max;

fn main(){
    input!{
        n: usize, 
        p: [(i32, i32); n], 
    }

    let mut length = 0;

    for (x1,y1) in p.iter(){
        for (x2, y2) in p.iter(){
            let x = x1 - x2;
            let y = y1 - y2;
            length = max(length, x*x + y*y);
        }
    }

    println!("{}", (length as f64).sqrt());

}