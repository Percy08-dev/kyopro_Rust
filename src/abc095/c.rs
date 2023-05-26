use proconio::input;
use std::cmp::{min, max};

fn main(){
    input!{
        a: i32, b: i32, c: i32, 
        x: i32, y: i32, 
    }

    let res1 = min(x, y) * c * 2 + max(0, (x-y)*a) + max(0, (y-x)*b);
    let res2 = a*x + b*y;
    let res3 = max(x, y) * c * 2;

    println!("{}", min(res1, min(res2, res3)));

}