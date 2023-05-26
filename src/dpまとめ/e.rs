use proconio::{input, fastout};
use std::cmp::min;

#[fastout]
fn main(){
    input!{
        N: usize, W:usize, 
        item:[(usize, usize); N], 
    }

    let weight:Vec<usize> = item.iter().map(|(wt, _)| *wt).collect();
    let value:Vec<usize> = item.iter().map(|(_, vt)| *vt).collect();

    let vmax = *value.iter().max().unwrap();
    let vmax = 100_000;

    let mut dp = vec![vec![1_000_000_001; vmax + 1]; N+1];
    dp[0][0] = 0;

    for i in 1..=N {
        for v in 0..=vmax {
            if v.checked_sub(value[i-1]).is_some() {
                dp[i][v] = min(dp[i][v], dp[i-1][v - value[i-1]] + weight[i-1]);
            }
            dp[i][v] = min(dp[i][v], dp[i-1][v]);
        }
    }


    let mut ans = 0;
    for (i, w) in dp[N].iter().enumerate() {
        if *w <= W {
            ans = i;
        }
    }

    println!("{}", ans);

}