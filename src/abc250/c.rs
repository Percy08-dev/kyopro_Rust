use proconio::{input, fastout};
use std::iter::FromIterator;
use std::collections::HashMap;

#[fastout]
fn main(){
    input!{
        n: usize, q: usize, 
        mut x:[usize; q],
    }

    let mut ball:Vec<usize> = Vec::from_iter(0..=n);
    let mut m: HashMap<usize, usize> = HashMap::new();

    for i in ball.iter() {
        m.insert(*i, *i);
    }
    
    for i in x.into_iter() {
        let idx = *m.get(&i).unwrap();
        if idx == n {
            let tmp = ball[idx-1];
            m.insert(i, idx-1);
            m.insert(tmp, idx);
            ball.swap(idx, idx-1);
        }else {
            let tmp = ball[idx+1];
            m.insert(i, idx+1);
            m.insert(tmp, idx);
            ball.swap(idx, idx+1);
        }
    }


    for i in ball.into_iter().skip(1){
        print!("{} ", i);
    }

    println!();

}