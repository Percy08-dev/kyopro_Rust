use proconio::{input, fastout};
use std::collections::HashSet;
use std::iter::FromIterator;

#[fastout]
fn main(){
    input!{
        n: usize, 
        mut p: [(i64, i64); n], 
    }

    p.sort_unstable_by(|x, y| x.0.cmp(&y.0));
    let x = vec![p[0], p[1], p[2], p[n-3], p[n-2], p[n-1]];
    
    p.sort_unstable_by(|x, y| x.1.cmp(&y.1));
    let y = vec![p[0], p[1], p[2], p[n-3], p[n-2], p[n-1]];

    let mut ans = HashSet::<i64>::new();

    for (x1, y1) in x.iter() {
        for (x2, y2) in y.iter() {
            if x1 == x2 && y1 == y2 {
                continue;
            }
            ans.insert((x1-x2).abs().max((y1-y2).abs()));
        }
    }

    let mut ans = Vec::from_iter(ans.into_iter());

    ans.sort_unstable();
    // println!("{:?}", ans);
    println!("{}", ans[ans.len()-2]);
}
