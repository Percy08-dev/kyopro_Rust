use proconio::{input, fastout};

const M: i64 = 998244353;

#[fastout]
fn main(){
    input!{
        n: usize, 
    }

    let mut dp: Vec<Vec<i64>> = vec![vec![0;9];n];

    for i in 0..9{
        dp[0][i] = 1;
    }

    for i in 1..n{
        for j in 0..9{
            dp[i][j] = if j == 0 {
                dp[i-1][j] + dp[i-1][j+1]
            }else if j == 8{
                dp[i-1][j-1] + dp[i-1][j]
            }else{
                dp[i-1][j-1] + dp[i-1][j] + dp[i-1][j+1]
            } % M;
        }
    }

    println!("{}", dp[n-1].iter().sum::<i64>()%M);
}