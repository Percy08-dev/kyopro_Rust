use proconio::{input, fastout};
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use std::cmp::max;
use num::pow;

#[fastout]
fn main(){
    input!{
        n: usize, k:usize, 
        s:[String; n], 
    }

    let s:Vec<HashSet<char>> = s.into_iter().map(|x| HashSet::from_iter(x.chars())).collect();
    let mut ans = 0;
    for i in 0..pow(2, n) {
        let mut counter:HashMap<char, usize> = HashMap::new();
        for shift in 0..n {
            if i >> shift & 1 == 1 {
                for &c in s[shift].iter() {
                    *counter.entry(c).or_insert(0) += 1;
                }
            }
        }
        ans = max(ans, counter.into_iter().filter(|&x| x.1 == k ).count())
    }

    println!("{}", ans);
}