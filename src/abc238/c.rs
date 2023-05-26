use proconio::{input, fastout};
use std::cmp::min;
use num::pow;

#[fastout]

fn s(x: u128)->u128{
    x*(x+1)/2
}

fn main(){
    input!{
        n: u128, 
    }
    let p =  998244353;
    let mut ans = 0;
    for i in 1..=((n as f64).log10() as usize)+1{
        let x = min(n, pow(10, i) - 1) - (pow(10, i-1) - 1);
        ans = (ans + s(x)) % p;
    }
    


    println!("{}", ans);

}