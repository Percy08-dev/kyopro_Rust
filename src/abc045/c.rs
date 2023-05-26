use proconio::{input, fastout};
use std::collections::VecDeque;
use std::iter::FromIterator;

#[fastout]
fn main(){
    input!{
        s1: String, 
        s2: String, 
        s3: String, 
    }

    let mut d1 = VecDeque::from_iter(s1.chars());
    let mut d2 = VecDeque::from_iter(s2.chars());
    let mut d3 = VecDeque::from_iter(s3.chars());
    let mut tmp = d1.pop_front().unwrap();
    let ans;

    loop {
        if tmp == 'a' {
            if d1.is_empty() {
                ans = "A";
                break;
            }else{
                tmp = d1.pop_front().unwrap();
            }
        }else if tmp == 'b' {
            if d2.is_empty() {
                ans = "B";
                break;
            }else {
                tmp = d2.pop_front().unwrap();
            }
        }else if d3.is_empty() {
            ans = "C";
            break;
        }else {
            tmp = d3.pop_front().unwrap();
        }
    }

    println!("{}", ans);

}