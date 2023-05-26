use proconio::{input, fastout};
use std::cmp::max;
use std::collections::HashSet;
use std::iter::FromIterator;

use std::hash::BuildHasher;

#[fastout]
fn main(){
    input!{
        n: usize, 
        a: [usize; n], 
    }

    let nums:HashSet<usize> = HashSet::from_iter(a.clone().into_iter());
    let mut ans = 0;

    for i in nums.into_iter() {
        let mut tmp = 0;
        for j in a.iter() {
            if *j >= i {
                tmp += i;
            }else{
                ans = max(ans, tmp);
                tmp = 0;
            }
        }   
        ans = max(ans, tmp);
    }

    println!("{}", ans);


}