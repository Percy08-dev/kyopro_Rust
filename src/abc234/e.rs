use proconio::{input, fastout};
use std::{collections::HashSet, hash::Hash};

#[fastout]
fn main(){
    input!{
        num: String
    }
    let x = num.parse::<u64>().unwrap();
    let snum: Vec<char> = num.chars().collect();
    let mut anses: HashSet<u64> = HashSet::new();
    
    for first in 1..=9{
        for dif in -8..=9{
            let mut tmp = String::new();
            for i in 0..num.len() as i32 {
                if first - dif * i > -10 && first - dif * i < 10{
                    tmp.push((first - dif * i).to_string().chars().next().unwrap());
                }
            }
            if tmp.len() == num.len() {
                let test = tmp.parse::<u64>();
                match test {
                    Ok(test)=>anses.insert(test),
                    Err(_) => continue, 
                };
                
            }
        }
    }
    
    println!("{}", anses.iter().filter(|i| **i >= x).min().unwrap())
}