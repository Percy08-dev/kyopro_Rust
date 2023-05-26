use proconio::input;

fn main(){
    input!{
        n: usize, 
        mut a: [i64; n], 
        mut b: [i64; n], 
    }

    a.sort_unstable();
    b.sort_unstable();

    let mut complain: i64 = 0;

    for i in 0..n{
        complain += (a[i] - b[i]).abs();
    }

    println!("{}", complain);

}