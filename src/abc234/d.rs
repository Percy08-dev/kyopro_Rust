use proconio::{input, fastout};
use std::collections::BinaryHeap;

#[fastout]
fn main(){
    input!{
        n: usize, k: usize, 
        p: [i32; n], 
    }

    let mut que: BinaryHeap<i32> = BinaryHeap::with_capacity(n);

    for i in p.iter().take(k){
        que.push(-*i);
    }
    let top = que.pop().unwrap();
    que.push(top);
    println!("{}", -top);

    for i in k..n{
        let top = que.pop().unwrap();
        que.push(top);
        // println!("{:?}, {}", que, top);
        if -top < p[i]{
            que.pop();
            que.push(-p[i]);
        }
        let top = que.pop().unwrap();
        que.push(top);
        println!("{}", -top);
    }
}