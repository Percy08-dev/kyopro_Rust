use proconio::{input, fastout};
use std::cmp::max;


#[fastout]
fn main(){
    input!{
        n: usize, 
        s: String, 
    }

    let M = 1_000_000_000 + 7;
    let s1:Vec<char> = s.chars().collect();
    let s2:Vec<char> = "atcoder".chars().collect();

    let mut dp = vec![vec![0; n+1]; 7+1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..8 {
            dp[j][i+1] = (dp[j][i+1] + dp[j][i]) % M;
            if j < s2.len() && s1[i] == s2[j] {
                dp[j+1][i+1] = (dp[j+1][i+1] + dp[j][i]) % M;
            }
        }
    }
    /*
    for i in dp.iter() {
        println!("{:?}", i);
    }
    */

    println!("{}", dp[7][n]);


}