use proconio::{input, fastout};
use std::cmp::max;

#[fastout]
fn main(){
    input!{
        n: usize, 
        a: [usize; n], 
    }

    let mut angle: Vec<usize> = vec![0];

    for i in a.into_iter(){
        angle = angle.iter().map(|x| (x + i) % 360).collect();
        angle.push(0);
    }

    angle.sort_unstable();

    let mut tmp1: usize = 360;
    let mut tmp2: usize;
    let mut ans: usize = 0;
    while !angle.is_empty(){
        tmp2 = angle.pop().unwrap();
        ans = max(ans, tmp1 - tmp2);
        tmp1 = tmp2;
    }

    println!("{}", ans);
    
}