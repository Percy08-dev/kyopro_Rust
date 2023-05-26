use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, 
        a: [usize; n], 
    }

    let mut r4 = 0;
    let mut r2 = 0;
    let mut r1 = 0;

    for i in a.into_iter() {
        if i % 4 == 0 {
            r4 += 1;
        }else if i % 2 == 0 {
            r2 += 1;
        }else {
            r1 += 1;
        }
    }

    let tmp1 = r4 - r1 + 1;
    let tmp2 = r2 % 2;

    if tmp1 >= tmp2 {
        println!("Yes");
    }else {
        println!("No");
    }
}