use num::pow;
use std::cmp::max;
use proconio::{input, fastout};

#[fastout]

fn Bit_FS(n1:&mut Vec<char>, n2:&mut Vec<char>, n:& Vec<char>, i:usize)->usize{
    for j in 0..n.len(){
        if i >> j & 1 == 1{
            n1.push(n[j]);
        }else{
            n2.push(n[j]);
        }
    }

    n1.sort_unstable();
    n2.sort_unstable();
    let num1: usize = n1.iter().rev().collect::<String>().parse().unwrap_or(0);
    let num2: usize = n2.iter().rev().collect::<String>().parse().unwrap_or(0);

    num1 * num2
}


fn main(){
    input!{
        n: String, 
    }

    let n: Vec<char> = n.chars().into_iter().collect();
    let mut n1:Vec<char> = Vec::with_capacity(10);
    let mut n2:Vec<char> = Vec::with_capacity(10);
    let mut ans: usize = 0;

    for i in 0..pow(2, n.len()){
        ans = max(ans, Bit_FS(&mut n1, &mut n2, &n, i));
        n1.clear();
        n2.clear();
    }
    

    
    println!("{}", ans);

}