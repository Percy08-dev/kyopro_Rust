use proconio::input;
use std::cmp::{max, min};

fn main(){
    input!{
        h: usize, w: usize, 
    }

    let mut res = 0;
    if h == 1 || w == 1{
        res =  max(h, w);
    }else{
        res = ((h+1)/2)*((w+1)/2);
    }
    println!("{}", res);
}