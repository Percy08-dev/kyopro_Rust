use proconio::{input, fastout};
use std::collections::HashSet;

#[fastout]
fn main(){
    input!{
        n: usize, x: usize, 
        a: [usize; n], 
    }

    let mut used:HashSet<usize> = HashSet::new();
    let mut now = x - 1;
    let mut cnt = 0;


    while used.get(&now).is_none() {
        cnt += 1;
        used.insert(now);
        now = a[now] - 1;
    }

    println!("{}", cnt);


}