use proconio::{input, fastout};
use num::integer::lcm;

fn f(start: usize, end: usize, r:usize) -> usize {
    let last = end/r*r;
    // println!("{} {}", end, last);
    let res = if last < r {
        0
    }else{
        (start+last)*last/r / 2
    };

    res
}


#[fastout]
fn main(){
    input!{
        n: usize, 
        a: usize, 
        b: usize,
    }

    let mut s = (1+n)*n/2;
    let l = lcm(a, b);
    let sa = f(a, n, a);
    let sb = f(b, n, b);
    let slcm = f(l, n, l);
    
    println!("{}", s + slcm - sa- sb);

}