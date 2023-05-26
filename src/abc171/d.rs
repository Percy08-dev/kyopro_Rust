use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main(){
    input!{
        n: usize, 
        a: [i64; n], 
        q: usize, 
        c: [(i64, i64); q], 
    }

    let mut nums:HashMap<i64, i64> = HashMap::new();
    let mut ans = a.iter().sum::<i64>();

    for i in a.into_iter() {
        *nums.entry(i).or_insert(0) += 1;
    }

    for (a, b) in c.into_iter() {
        let tmp = *nums.get(&a).unwrap_or(&0);
        *nums.entry(b).or_insert(0) += tmp;
        nums.insert(a, 0);

        ans += (b - a) * tmp;


        println!("{}", ans);
    }



}