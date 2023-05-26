use itertools::Itertools;
use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, 
        s: String, 
    }
    let mut s1 = s.chars().take(n/2).collect_vec();
    let mut s2 = s.chars().rev().take(n/2).collect_vec();
    let mut ans = "Yes";
    
    println!("{}", ans);
    println!("{:?}, {:?}", s1, s2);
}