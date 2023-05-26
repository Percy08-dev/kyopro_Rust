use proconio::{input, fastout};

fn f(n: usize) -> f64 {
    let x: f64 = n as f64;
    (1.0 + x) / 2.0
}

#[fastout]
fn main(){
    input!{
        n: usize, k: usize, 
        p: [usize; n], 
    }
    
    let u: Vec<f64> = p.iter().map(|x| f(*x)).collect();
    let mut u_acc: Vec<f64> = Vec::with_capacity(n);    // 累積和
    u_acc.push(u[0]);

    for i in u.into_iter().skip(1) {
        let x = *u_acc.last().unwrap();
        u_acc.push(x + i);
    }

    let mut ans = u_acc[k-1];
    for i in 1..=n-k {
        ans = ans.max(u_acc[i + k - 1] - u_acc[i-1]);
    }
    

    println!("{}", ans);
}