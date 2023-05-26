use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, m: usize, 
        mut a: [i32; n + 1], 
        mut c: [i32; n + m + 1], 
    }
    
    a = a.into_iter().rev().collect();
    c = c.into_iter().rev().collect();
    let mut ans = vec![0; m + 1]; 
    /* 多項式の除算で行けそう */
    for i in 0..=m {
        let k = c[i] / a[0];
        ans[i] = k;
        for j in 0..=n {
            c[i + j] -= a[j] * k;
        }
    }

    for i in ans.into_iter().rev() {
        print!("{} ", i);
    }
    println!();

}