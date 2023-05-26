use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, 
        a: [i64; n], 
        x: i64, 
    }

    let s: i64 = a.iter().sum();
    let mut ans = 0;
    let mut mem = x - x % s;
    ans += x / s * n as i64;
    // println!("{} {}", ans, mem);
    for i in a.into_iter(){
        mem += i;
        ans += 1;
        if mem > x{
            break;
        }
    }

    println!("{}", ans);
}