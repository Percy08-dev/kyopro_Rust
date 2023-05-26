use proconio::{input, fastout};
use std::cmp::min;

#[fastout]
fn main(){
    input!{
        n: usize, m: usize, 
        mut a: [i64; n], 
        mut b: [i64; m], 
    }

    a.sort_unstable();
    b.sort_unstable();

    let mut idxa = 0;
    let mut idxb = 0;
    let mut ans = 1_000_000_000;
    while idxa < n && idxb < m {
        ans = min(ans,  (a[idxa] - b[idxb]).abs());
        if a[idxa] - b[idxb] < 0{
            idxa += 1;
        }else{
            idxb += 1;
        }
    }

    println!("{}", ans);

}