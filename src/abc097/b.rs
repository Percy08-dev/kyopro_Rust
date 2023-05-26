use proconio::{input, fastout};
use std::cmp::max;
use num::integer::Roots;
use num::pow;

#[fastout]
fn main(){
    input!{
        x: i64, 
    }

    let upper = x.sqrt();
    let mut ans = 1;

    for i in 2..=upper {
        for j in 1..=10 {
            let tmp = pow(i, j);
            if tmp <= x {
                ans = max(ans, tmp);
            }else{
                break;
            }
        }
    }

    println!("{}", ans);
}