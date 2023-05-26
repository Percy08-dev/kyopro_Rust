use proconio::input;
use std::{collections::HashSet};

fn main(){
    input!{
        n: usize, 
        S: [String; n],
    }

    let mut map:HashSet<String> = HashSet::new();
    
    for i in 0..S.len(){
        if map.get(&S[i]) == None{
            println!("{}", i+1);
            map.insert(S[i].clone());
        }
    }
}