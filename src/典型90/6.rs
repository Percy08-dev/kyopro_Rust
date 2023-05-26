use itertools::Itertools;
use proconio::{input, fastout};
use std::collections::HashMap;
use std::cmp::min;

#[fastout]
fn main(){
    input!{
        n: usize, k: usize, 
        s:String, 
    }

    let s = s.chars().collect_vec();
    let alpha = "abcdefghijklmnopqrstuvwxyz".chars().collect_vec();
    let alpha_map:HashMap<char, usize> = "abcdefghijklmnopqrstuvwxyz".chars().enumerate().map(|(i, c)| (c,i)).collect();

    let mut table = vec![vec![1_000_000_000; alpha.len()]; s.len()];
    let mut ans = Vec::new();

    for i in 0..s.len() {
        for (j, c) in s.iter().skip(i).enumerate() {
            let idx = *alpha_map.get(c).unwrap();
            table[i][idx] = min(table[i][idx], j);
        }
    }

    
    for i in table.iter() {
        println!("{:?}", i);
    }
    


    let mut i = 0;
    while ans.len() < k {
        println!("{}", i);
        for (alp, c_num) in table[i].iter().enumerate() {
            if *c_num < s.len() - k + ans.len() {
                println!("nth_after:{} lim:{} char:{}", c_num, s.len() - k , alpha[alp]);
                ans.push(alpha[alp]);
                i = i + *c_num + 1;
                break;
            } 
        }
    }


    println!("{:?}", ans);

}