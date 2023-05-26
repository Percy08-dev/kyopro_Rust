use itertools::Itertools;
use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, 
        mut a: [usize; n], 
    }

    let mut ans = 0;

    let x = a.iter().enumerate().filter(|&(i, n)| i+1 == *n).map(|(_i, &n)| n).collect_vec();
    ans += (1 + x.len() - 1) * (x.len() - 1) / 2;

    for i in 0..n {
        let idx = i + 1;
        let k_v = a[i] - 1;
        if idx == a[k_v] && i < k_v {
            ans += 1;
        }
    }

    
    println!("{}", ans);

}