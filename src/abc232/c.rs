use proconio::{input, fastout};
use itertools::Itertools;

#[fastout]
fn main(){
    input!{
        n: usize, m: usize, 
        s1: [(usize, usize); m], 
        s2: [(usize, usize); m], 
    }

    let mut t1 : Vec<Vec<bool>> = vec![vec![false; n]; n]; 
    let mut t2 : Vec<Vec<bool>> = vec![vec![false; n]; n]; 

    for i in 0..m {
        t1[s1[i].0-1][s1[i].1-1] = true;
        t1[s1[i].1-1][s1[i].0-1] = true;
        t2[s2[i].0-1][s2[i].1-1] = true;
        t2[s2[i].1-1][s2[i].0-1] = true;
    }

    let mut ok = true;

    for p in (0..n).permutations(n) {
        ok = true;
        for i in 0..n {
            for j in 0..n {
                if t1[i][j] != t2[p[i]][p[j]] {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
        }
        if ok {
            break;
        }
    }

    if ok {
        println!("Yes");
    }else{
        println!("No");
    }



}