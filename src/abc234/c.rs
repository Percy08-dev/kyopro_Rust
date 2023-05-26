use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        k: usize, 
    }

    let mut num = k;
    let mut res: Vec<char> = Vec::with_capacity(64);

    while num > 0 {
        if num & 1 == 1{
            res.push('2');
        }else{
            res.push('0');
        }
        num >>= 1;
    }
    let ans: String =  res.iter().rev().collect();
    println!("{}", ans);
}
