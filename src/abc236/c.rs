use proconio::{input, fastout};
use std::collections::HashSet;

#[fastout]
fn main(){
    input!{
        n: usize, m: usize, 
        s: [String; n], 
        t: [String; m], 
    }

    let mut t_set:HashSet<String> = HashSet::new();

    for i in t.into_iter(){
        t_set.insert(i);
    }

    for i in s.iter(){
        if t_set.get(i).is_none() {
            println!("No");
        }
        else{
            println!("Yes");
        }
    }

}