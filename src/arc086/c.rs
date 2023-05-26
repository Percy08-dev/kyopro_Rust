use proconio::{input, fastout};
use std::collections::HashMap;
use std::iter::FromIterator;

#[fastout]
fn main(){
    input!{
        n: usize, k: usize, 
        a: [i32; n], 
    }

    let mut counter: HashMap<i32, usize> = HashMap::new();
    let mut ans = 0;

    for i in a.into_iter() {
        *counter.entry(i).or_insert(0) += 1;
    }

    let mut counter: Vec<(i32, usize)> = Vec::from_iter(counter);
    let mut t_cnt = counter.len();
    counter.sort_unstable_by(|x, y| x.1.cmp(&y.1).reverse());

    while t_cnt > k {
        let tmp = counter.pop().unwrap();
        t_cnt -= 1;
        ans += tmp.1;
        // println!("{:?}", tmp);
    }


    println!("{}", ans);




}