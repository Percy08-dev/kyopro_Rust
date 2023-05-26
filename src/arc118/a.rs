use proconio::{input, fastout};

fn f(x:i64, t:i64) -> i64 {
    (100+t)*x/100
}

#[fastout]
fn main(){
    input!{
        t: i64, 
        n: i64,
    }

    let mem:Vec<i64> = (1..=100).into_iter().map(|x| f(x, t)).collect();
    println!("{:?}", mem);
}