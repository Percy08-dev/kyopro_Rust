use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        mut n: usize, 
    }

    let mut ans: String = String::new();
    
    while n > 0 {
        if n % 2 == 0 {
            n /= 2;
            ans.push('B');
        }else{
            n -= 1;
            ans.push('A');
        }
    }

    ans = ans.chars().rev().collect();

    println!("{}", ans);
}