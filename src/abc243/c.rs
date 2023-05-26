use proconio::{input, fastout};
use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};
use std::iter::FromIterator;

#[fastout]
fn main(){
    input!{
        n: usize, 
        points: [(usize, usize); n], 
        s: String, 
    }

    let mut R_collision: HashMap<usize, usize> = HashMap::new();
    let mut L_collision: HashMap<usize, usize> = HashMap::new();
    let s = s.chars().collect::<Vec<char>>();
    let mut ans = "No";

    for i in 0..n {
        if s[i] == 'R' {
            let tmp = R_collision.get(&points[i].1);
            if tmp.is_none() {
                R_collision.insert(points[i].1, points[i].0);
            }else {
                R_collision.insert(points[i].1, min(*tmp.unwrap(), points[i].0));
            }
        }else{
            let tmp = L_collision.get(&points[i].1);
            if tmp.is_none() {
                L_collision.insert(points[i].1, points[i].0);
            }else {
                L_collision.insert(points[i].1, max(*tmp.unwrap(), points[i].0));
            }
        }
    }

    let tmp_keys1:HashSet<usize> = HashSet::from_iter(R_collision.iter().map(|x| *x.0));
    let tmp_keys2:HashSet<usize> = HashSet::from_iter(L_collision.iter().map(|x| *x.0));
    let keys = tmp_keys1.intersection(&tmp_keys2);

    for i in keys {
        if R_collision[i] < L_collision[i] {
            ans = "Yes";
            break;
        }
    }

    println!("{}", ans);
}