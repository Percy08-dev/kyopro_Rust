use itertools::Itertools;
use proconio::{input, fastout};
use std::collections::VecDeque;

fn pp(x:&Vec<usize>) {
    for i in x.iter() {
        print!("{} ", i);
    }
    println!();
}

fn is_sorted<T>(data: &[T]) -> bool 
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] < w[1])
}

fn ans_format(x:&Vec<usize>) -> String{
    let mut res = "".to_string();
    for i in x.iter() {
        res = format!("{} {}", res, i.to_string())
    }
    res
}

fn ff(x:&mut Vec<usize>, m: usize) {
    let l = x.len();
    x[l-1] += 1;

    for i in (1..l).rev() {
        if x[i] > m {
            x[i-1] += 1;
            x[i] = 0;
        }
    }

    for i in 1..l {
        x[i] = if x[i-1] >= x[i] {
            x[i-1] + 1
        }else {
            x[i]
        }
    }
}



#[fastout]
fn main(){
    input!{
        n: usize, m: usize, 
    }

    let mut que = VecDeque::new();
    let tmp = (1..=n).collect_vec();
    que.push_back(tmp);

    while !que.is_empty() {
        let mut x = que.pop_front().unwrap();
        if x[n-1] <= m {
            pp(&x);
        }
        ff(&mut x, m);
        if x[0] <= m {
            que.push_back(x);
        }
    }



}