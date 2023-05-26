use proconio::input;

fn main(){
    input!{
        n: usize, k: i64, 
        a: [i64; n], 
        b: [i64; n],
    }

    let mut s: i64 = 0;

    for i in 0..n{
        s += (a[i] - b[i]).abs();
    }

    if (k - s).abs() % 2 == 0 && s <= k{
        println!("Yes");
    }else {
        println!("No");
    }
}