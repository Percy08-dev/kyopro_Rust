use proconio::{input, fastout};

const M:usize = 1000_000_000 + 7;

#[fastout]
fn main(){
    input!{
        s: String, 
    }

    let mut dp = vec![0; 9];
    let t = "chokudai".to_string();

    dp[0] = 1;

    for c in s.chars().into_iter() {
        for i in 0..t.len() {
            if t.chars().nth(i).unwrap() == c {
                dp[i+1] += dp[i];
                dp[i+1] %= M;
                break;
            }
        }
    }

    println!("{}", dp[8]);
    println!("{:?}", dp);
}