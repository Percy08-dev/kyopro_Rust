use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, 
    }

    if ((n*n) as f64).log(2.0) < n as f64 {
        println!("Yes");
    }else{
        println!("No");
    }

}