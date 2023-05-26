use std::vec;

use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, 
        mut b: [usize; n], 
    }

    let mut a:Vec<usize> = vec![0];
    a.append(&mut b);

    let mut dp:Vec<Vec<usize>> = vec![vec![0; 10]; n+1];
    let M = 998244353;

    dp[1][a[1]] = 1;

    for i in 1..n{
        for j in 0..10{
            dp[i+1][(j + a[i+1])%10] = (dp[i][j] + dp[i+1][(j + a[i+1])%10]) % M;
            dp[i+1][(j * a[i+1])%10] = (dp[i][j] + dp[i+1][(j * a[i+1])%10]) % M;
        }
        /*
        for i in dp.iter(){
            println!("{:?}", i);
        }
        println!("\n");
        */
    }
    // println!("{:?}", dp);
    for i in dp[n].iter(){
        println!("{}", i);
    }
    

    

}