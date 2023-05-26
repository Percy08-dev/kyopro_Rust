use proconio::{input, fastout};
use num::integer::cbrt;
use std::collections::HashSet;
use std::usize;

fn hurui(n:usize) -> Vec<usize> {
    let mut b = vec![true; n + 1];
    let mut primes:Vec<usize> = Vec::new();

    b[0] = false;
    b[1] = false;
    
    for i in 0..n+1 {
        if !b[i] {
            continue;
        }
        primes.push(i);
        for j in (i*i..n+1).step_by(i) {
            b[j] = false;
        }
    }

    primes
}


// #[fastout]
fn main(){
    input!{
        n: usize,
    }

    let upper_prime = cbrt(n/2);
    let p = hurui(upper_prime);
    let mut like:HashSet<usize> = HashSet::new();

    
    for i in 0..p.len() {
        for j in i+1..p.len() {
            let tmp = if p[j] * p[j] * p[j] > usize::MAX / p[i] {
                break;
            }else{
                p[i] * p[j] * p[j] * p[j]
            };
            like.insert(tmp);
        }
    }
    

    println!("{}", like.len());
}