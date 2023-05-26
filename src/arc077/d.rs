use proconio::input;
use num::pow;

/* 未完成 */

fn cmb(n: usize, r: usize)->usize{
    let p: usize = 1_000_000_007; 
    1
}

fn main(){
    input!{
        n: usize, 
        a: [usize; n+1], 
    }
    let mut counter: Vec<Vec<usize>> = vec![vec![]; n];
    let p: usize = 1_000_000_007; 

    /* カウント */
    for i in a.iter().enumerate(){
        counter[i.1-1].push(i.0);
    }

    /* 2個の部分を探す */
    let index = counter.iter().max_by(|x, y| x.len().cmp(&y.len())).unwrap();
    let diff = index[1] - index[0];
    println!("{:?} {:?}", diff, index);
    for i in 1..=n+1{
        let mut res: usize = 0;
        res = cmb(n + 1, i);
        
        // println!("{} {}", i, res);
        println!("{}", res);
    }
}