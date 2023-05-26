use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main(){
    input!{
        n: usize, 
        a: [(usize, usize); n-1],
    }

    let mut counter:HashMap<usize, i32> = HashMap::with_capacity(n);

    for (i, j) in a.into_iter(){
        *counter.entry(i).or_insert(0) += 1;
        *counter.entry(j).or_insert(0) += 1;
    }

    let mut ans = "No";
    for (_, j) in counter.iter(){
        if *j == (n-1) as i32{
            ans = "Yes";
            break;
        }
    }

    println!("{}", ans);
}   