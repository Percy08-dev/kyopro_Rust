use proconio::input;
use std::cmp::{max};

fn main(){
    input!{
        n: usize, 
        mut p: [(i64, i64); n], 
    }

    let mut pp: Vec<i64> = p.iter().map(|&(x, y)| x + y).collect();
    let mut mp: Vec<i64> = p.iter().map(|&(x, y)| x - y).collect();
    
    pp.sort_unstable();
    mp.sort_unstable();

    let res = max(pp[n-1] - pp[0], mp[n-1] - mp[0]);

    println!("{}", res);
}