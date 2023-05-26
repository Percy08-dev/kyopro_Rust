use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, 
        a: [usize; n], 
    }

    let mut num = 1;
    let mut cnt = 0;
    
    for i in a.into_iter(){
        if i == num {
            cnt += 1;
            num += 1;
        }
    }
    if cnt == 0 {
        println!("{}", -1);
    }else {
        println!("{}", (n as i32) - cnt);
    }
}