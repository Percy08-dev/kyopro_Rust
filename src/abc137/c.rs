use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main(){
    input!{
        n: usize, 
        s:[String; n], 
    }

    let mut mozi: HashMap<String, usize> = HashMap::new();
    let mut cnt = 0;
    for i in s.into_iter() {
        let mut tmp:Vec<char> = i.chars().collect();
        tmp.sort_unstable();
        let tmp:String = tmp.iter().collect();
        if mozi.get(&tmp).is_some() {
            cnt += mozi.get(&tmp).unwrap();
            *mozi.entry(tmp).or_insert(1) += 1;
        }else {
            mozi.insert(tmp, 1);
        }
    }

    println!("{}", cnt);

}