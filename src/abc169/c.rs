use num::pow;
use proconio::input;

fn main(){
    input!{
        a: usize, b: f64, 
    }
    let mut b = b.to_string();
    let idx= b.len() - b.find('.').unwrap_or(b.len() - 1) - 1;
    b.retain(|c| c != '.');
    
    let b: usize = b.parse().unwrap();
    let res = a * b / pow(10, idx);
    println!("{}", res);
}