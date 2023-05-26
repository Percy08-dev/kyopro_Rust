use itertools::Itertools;
use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main(){
    input!{
        n:  usize, 
        s: [String; n], 
    }

    let mut counter:HashMap<String, usize> = HashMap::new();

    for i in s.iter() {
        let t = counter.get(i);
        if t.is_none() {
            println!("{}", i);
            counter.insert(i.to_string(), 1);
        }else{
            println!("{}({})", i, t.unwrap());
            *counter.get_mut(i).unwrap() += 1;
        }
    }


}