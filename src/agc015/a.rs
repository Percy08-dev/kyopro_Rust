use proconio::{input, fastout};
use std::cmp::max;

#[fastout]
fn main(){
    input!{
        n: i64, a: i64, b: i64, 
    }

    let ans = (a + b*(n-1)) - (a*(n-1) + b) + 1;
    println!("{}", max(0, ans));

}