use proconio::{*};
use std::{collections::HashMap, vec};

#[fastout]
fn main(){
    input!{
        n: usize, q: usize, 
        a: [usize; n], 
        query: [(usize, usize); q], 
    }
    let mut num_appearance: HashMap<usize, Vec<usize>> = HashMap::new();
    let tmp: Vec<usize> = Vec::new();


    for (i, j) in a.into_iter().enumerate(){
        num_appearance.entry(j).or_insert(Vec::new()).push(i+1);
    }
    
    for (x, k) in query.into_iter(){
        /* 
        let mem = num_appearance.get(&x).unwrap_or(&tmp);

        if mem.len() < k{
            println!("{}", -1);
        }else{
            println!("{}", mem[k-1] as i32);
        }
        */
        if let None = num_appearance.get(&x) {
            println!("-1");
            continue;
        }
        if num_appearance.get(&x).unwrap().len() < k {
            println!("-1");
        }else{
            println!("{}", num_appearance.get(&x).unwrap()[k-1]);
        }
    }
}