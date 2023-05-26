use proconio::{input, fastout};
use std::{collections::VecDeque};

#[fastout]
fn main(){
    input!{
        n: usize, 
        //s: [char; n], 
        s: String
    }
    let s = s + "L";
    let s:Vec<char> = s.chars().collect();
    let mut ans:VecDeque<usize> = VecDeque::with_capacity(n+1);

    for (i, j) in s.into_iter().enumerate().rev(){
        if j == 'L'{
            ans.push_back(i);
        }else{
            ans.push_front(i);
        }
        println!("{:?}", ans);
    }

    for i in ans.iter(){
        print!("{} ", i);
    }

}