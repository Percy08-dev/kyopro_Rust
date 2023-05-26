use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, 
        s: String, 
        t: String, 
    }

    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    let mut common = String::new();
    let mut start: usize = 0;

    for i in 0..n {
        if common.ends_with(s[i]) && s[i] != t[i - start] && common.len() == 1 {
            start += 1;
        }else if s[i] != t[i - start] {
            start = i + 1;
            common.clear();
        }else {
            common.push(s[i]);
        }
    }

    println!("{}", 2*n - common.len());
}