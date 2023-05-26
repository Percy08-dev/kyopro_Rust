use proconio::{input, fastout};
use std::collections::VecDeque;

#[fastout]
fn main(){
    input!{
        q: usize, 
    }

    let mut pipe: VecDeque<(i64, i64)> = VecDeque::new();

    for _i in 0..q {
        input! {b:i32}
        if b == 1 {
            input! {tmp: (i64, i64)}
            pipe.push_back(tmp);
        }else {
            input! {mut p:i64}
            let mut s = 0;
            while p > 0 {
                let mut tmp = pipe.pop_front().unwrap();
                if tmp.1 <= p {
                    p -= tmp.1;
                    s += tmp.0 * tmp.1;
                }else {
                    tmp.1 -= p;
                    s += tmp.0 * p;
                    p = 0;
                    pipe.push_front(tmp);
                }
            }
            println!("{}", s);
        }
    }

}