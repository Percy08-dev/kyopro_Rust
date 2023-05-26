use itertools::Itertools;
use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        o: String, 
        e: String, 
    }

    let mut ans: String = String::new();


    for i in o.chars().zip_longest(e.chars()){
        if i.has_left(){
            ans.push(i.clone().left().unwrap());
        }

        if i.has_right(){
            ans.push(i.right().unwrap());
        }
    }

    println!("{}", ans);
}