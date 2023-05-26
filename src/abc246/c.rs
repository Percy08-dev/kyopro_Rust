use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, mut k: i64, x: i64, 
        a:[i64; n], 
    }

    let mut ans = 0;
    let mut mem = Vec::new();

    for i in a.into_iter() {
        if k == 0 {
            ans += i;
        }else if i/x <= k {
            mem.push(i % x);
            k -= i/x;
        }else{
            ans += i - x*k;
            k = 0;
        }
    }
    mem.sort_unstable();

    if n as i64 - k > 0 { 
        ans += mem.iter().take(n - k as usize).sum::<i64>();
    }
    println!("{}", ans);
}