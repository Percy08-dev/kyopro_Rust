use proconio::input;
use num::integer::gcd;

fn main(){
    input!{
        a: u64, b: u64, c:u64, 
    }
    // let base = gcd(a, gcd(b, c));
    let tmp = gcd(a, b);
    let base = gcd(tmp, c);

    println!("{}", a/base + b/base + c/base - 3);
}