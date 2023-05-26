use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        t: usize, 
    }

    for _i in 0..t {
        input! {a: i64, s: i64}
        if (2*a <= s) && (a & (s - 2*a) == 0) {
            println!("Yes");
        }else{
            println!("No");
        }
    }
}