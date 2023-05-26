use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        mut a:[usize; 3], 
    }

    if a.iter().filter(|&x| x % 2 == 0).count() != 0 {
        println!("0");
    }else {
        a.sort_unstable();
        println!("{}", a[0]*a[1]);
    }

}