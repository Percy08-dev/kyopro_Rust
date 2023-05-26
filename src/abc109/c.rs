use proconio::{input, fastout};
use std::cmp::{min, max};
use num::integer::gcd;

#[fastout]
fn main(){
    input!{
        n: usize, start: usize, 
        mut x: [usize; n], 
    }

    if n == 1 {
        println!("{}", max(start, x[0]) - min(start, x[0]));
    }else {
        x.sort_unstable();

        let mut dif:Vec<usize> = Vec::with_capacity(n);
        dif.push(max(start, x[0]) - min(start, x[0]));

        for i in 0..n-1 {
            dif.push(x[i + 1] - x[i]);
        }

        let ans = (0..n-1).into_iter().map(|i| gcd(dif[i+1], dif[i])).min().unwrap();

        println!("{}", ans);
    }
}