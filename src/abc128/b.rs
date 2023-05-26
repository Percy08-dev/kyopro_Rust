use itertools::Itertools;
use proconio::{input, fastout};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator; 

#[fastout]
fn main(){
    input!{
        n: usize, 
        mut s: [(String, i32); n], 
    }

    let mut new_s: Vec<(String, i32, usize)> = Vec::new();

    for (i, j) in s.into_iter().enumerate(){
        new_s.push((j.0, j.1, i+1));
    }

    let tmp1: Vec<String> = new_s.iter().map(|x| x.clone().0).collect();
    let tmp2: HashSet<String> = HashSet::from_iter(tmp1);
    let mut citys: Vec<String> = Vec::from_iter(tmp2);

    citys.sort_unstable();

    for i in citys.into_iter() {
        let mut tmp3 = new_s.iter().filter(|x| x.0 == i).collect_vec();
        tmp3.sort_unstable_by(|x, y| x.1.cmp(&y.1).reverse());
        for j in tmp3.iter() {
            println!("{}", j.2);
        }
    }


}