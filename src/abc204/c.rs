use proconio::{input, fastout};
use std::collections::{HashMap, HashSet};

#[fastout]
fn main(){
    input!{
        n: usize, 
        m: usize, 
        tmp: [(usize, usize); m]
    }

    let mut way:HashMap<usize, Vec<usize>> = HashMap::new();
    for i in tmp.into_iter() {
        way.entry(i.0).or_insert(Vec::new()).push(i.1);
    }

    let mut mem:Vec<usize> = Vec::new();
    let mut used:HashSet<(usize, usize)> = HashSet::new();
    let mut ans:HashSet<(usize, usize)> = HashSet::new();

    for start in 1..=n {
        ans.insert((start, start));
        mem.extend(way.get(&start).unwrap_or(&vec![]).clone());

        while !mem.is_empty() {
            let end = mem.pop().unwrap();

            if used.get(&(start, end)).is_some() {
                continue;
            }

            ans.insert((start, end));
            used.insert((start, end));
            mem.extend(way.get(&end).unwrap_or(&vec![]).clone())
        }
        used.clear();
    }


    println!("{}", ans.len());
}