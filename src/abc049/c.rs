use itertools::Itertools;
use proconio::{input, fastout};

// #[fastout]
fn main(){
    input!{
        s:String, 
    }
    /* 
    let s = s.chars().collect_vec();

    let s1 = "dream".chars().collect_vec();    
    let s2 = "dreamer".chars().collect_vec();   
    let s3 = "erase".chars().collect_vec();     
    let s4 = "eraser".chars().collect_vec();    
    */
    let model = vec!["dream", "dreamer", "erase", "eraser"];

    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;
    
    



    if dp[s.len()] {
        println!("YES");
    }else {
        println!("NO");
    }


}