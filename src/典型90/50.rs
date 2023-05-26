use num::pow;
use proconio::input;

fn main(){
    input!{
        n: usize, l:usize, 
    }
    let MOD: u128 = pow(10, 9) + 7;

    let mut dp:Vec<u128> = vec![1; n+1];

    for i in 1..=n{
        if i < l{
            dp[i] = dp[i-1];
        }else{
            dp[i] = (dp[i-1] + dp[i-l]) % MOD;
        }
        println!("{:?}", dp);
    }
    // println!("{:?}", dp);
    println!("{}", dp[n]);
}