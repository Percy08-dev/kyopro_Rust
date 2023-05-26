use im_rc::vector::Iter;
use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main(){
    input!{
        n: usize, k: i64, 
        a: [i64; n], 
    }

    let mut cnt = 0;
    let mut nums: HashMap<i64, i64> = HashMap::with_capacity(n);
    let mut a_sum: Vec<i64> = Vec::with_capacity(n+1);
    a_sum.push(0);

    for (i, j) in a.iter().enumerate(){
        a_sum.push(a_sum[i] + j);
    }
    
    for i in 1..n+1{
        *nums.entry(a_sum[i-1]).or_insert(0) += 1;
        let tmp = match nums.entry(a_sum[i] - k) {
            std::collections::hash_map::Entry::Occupied(x) => *x.get(),
            std::collections::hash_map::Entry::Vacant(_) => 0,
        };
        // println!("{:?} {}", nums, a_sum[i]-k);
        cnt += tmp;
    }

    

    println!("{}", cnt);

}