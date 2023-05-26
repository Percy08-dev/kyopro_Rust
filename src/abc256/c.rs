use proconio::{input, fastout};
use std::cmp::min;

#[fastout]
fn main(){
    input!{
        h1: i32, 
        h2: i32, 
        h3: i32, 
        w1: i32,
        w2: i32,
        w3: i32,
    }

    let mut ans = 0;

    for i11 in 1..min(h1, w1) {
        for i12 in 1..min(h1 - i11, w2) {
            let i13 = h1 - i11 - i12;
            for i21 in 1..min(h2, w1 - i11) {
                let i31 = w1 - i11 - i21;
                for i22 in 1..min(h2 - i21, w2 - i12) {
                    let i23 = h2 - i21 - i22;
                    let i_32 = w2 - i12 - i22;
                    if (h3 - i31 - i_32) == (w3 - i13 - i23) && (h3 - i31 - i_32) > 0{
                        ans += 1;
                    }
                }
            }
        }
    }


    println!("{}", ans);
}

/*
println!("{} {} {}", i11, i12, i13);
println!("{} {} {}", i21, i22, i23);
println!("{} {} {}\n", i31, i_32, i33);

*/