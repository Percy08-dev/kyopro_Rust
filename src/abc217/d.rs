use proconio::{input, fastout};
use std::{collections::BTreeSet, hash::{Hash, Hasher}};
use rustc_hash::FxHasher;

#[fastout]
fn main(){
    input!{
        L: usize, Q:usize, 
    }
    let mut woods:BTreeSet<usize> = BTreeSet::new();
    let mut Fx = FxHasher::default();
    woods.hash(&mut Fx);
    woods.insert(0);
    woods.insert(L);

    for _i in 0..Q {
        input!{c:i32, x:usize}
        if c == 1 {
            woods.insert(x);
        }else{
            let left = *woods.range(0..x).last().unwrap();
            let right = *woods.range(x..=L).next().unwrap();
            println!("{}", right - left);
        }
    }

}