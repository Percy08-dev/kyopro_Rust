use proconio::{input, fastout};

fn f(num:usize) {
    let mut n = num;
    let mut ans: Vec<char> = Vec::new();
    let s = "abcdefghijklmnopqrstuvwxyz".to_string();

    while n > 0 {
        n -= 1;
        ans.push(s.chars().nth(n%26).unwrap());
        n /= 26;
    }
    
    // println!("{} {}", num, ans.iter().rev().collect::<String>());
    println!("{}", ans.iter().rev().collect::<String>());

}

#[fastout]
fn main(){
    input!{
        n: usize, 
    }

    f(n);

}