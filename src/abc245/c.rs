use proconio::{input, fastout};
use std::i64;

#[fastout]
fn main(){
    input!{
        n: usize, k: i64, 
        a: [i64; n], 
        b: [i64; n], 
    }

    let mut mem = (a[0], b[0]); /* [a, b] */

    for i in 1..n {
        let tmp1 = if (mem.0 - a[i]).abs() <= k || (mem.1 - a[i]).abs() <= k {
            a[i]
        }else{
            i64::MAX
        };
        let tmp2 = if (mem.0 - b[i]).abs() <= k || (mem.1 - b[i]).abs() <= k {
            b[i]
        }else{
            i64::MAX
        };
        mem = (tmp1, tmp2);
    }

    if mem.0 == i64::MAX && mem.1 == i64::MAX {
        println!("No");
    }else{
        println!("Yes");
    }

}

/*
4 100
1 1000 1000 1
10000 1 1 10000

*/