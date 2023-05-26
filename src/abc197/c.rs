use num::pow;
use std::cmp::min;
use std::usize;
use proconio::input;

fn main(){
    input!{
        n: usize, a: [usize; n], 
    }

    let mut res: usize = usize::MAX;

    for i in 0..pow::<usize>(2, n){
        let mut mem: usize = 0;
        let mut tmp: usize = 0;
        for j in 0..n{
            if i >> j & 1 == 0{
                tmp |= a[j];
            }else{
                mem ^= tmp;
                tmp = a[j];
            }
        }
        mem ^= tmp;
        res = min(res, mem);
    }

    println!("{}", res);
}