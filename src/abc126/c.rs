use proconio::{input, fastout};

fn f(x: i32, k: f64)->i32{
    let mut x = x as f64;
    let mut cnt = 0;
    while x < k {
        x *= 2.0;
        cnt += 1;
    }
    cnt
}

#[fastout]
fn main(){
    input!{
        n: f64, k: f64, 
    }

    let mut ans: f64 = 0.0;

    if n >= k {
        ans += (n-k)/n;
    }

    let x = n.min(k) as i32;
    for i in 1..=x {
        let t = f(i, k);
        ans += 0.5f64.powi(t) * (1.0/n);
    }

    println!("{}", ans);
}