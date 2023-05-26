use proconio::{input, fastout};
use std::cmp::max;

#[fastout]

fn main(){
    input!{
        s: String, 
    }
    
    let mut nums: Vec<i64> = vec![0; s.len() + 1];

    for (i, j) in s.chars().enumerate() {
        if j == '<' {
            nums[i+1] = nums[i+1].max(nums[i] + 1);
        }
    }

    let x = nums.len()-1;

    for (i, j) in s.chars().rev().enumerate() {
        if j == '>' {
            nums[x - i - 1] = nums[x - i - 1].max(nums[x - i] + 1);
        }
    }
    
    println!("{}", nums.iter().sum::<i64>());
}