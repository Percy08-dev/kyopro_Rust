use num_integer::Roots;
use proconio::{input, fastout};
use std::{collections::HashSet};


fn primes(num: usize)->Vec<usize>{
    // let upper = num.sqrt() + 1;
    let mut b_res: Vec<bool> = vec![true; num + 1]; // 0 ~ num -> +2
    let mut res:Vec<usize> = Vec::with_capacity(num);
    b_res[0] = false;
    b_res[1] = false;

    for i in 2..=num{
        if !b_res[i] {
            continue;
        }
        let mut x = i*2;
        while x <= num {
            b_res[x] = false;
            x += i;
        }
    }
    
    for i in 2..=num{
        if b_res[i] {
            res.push(i);
        }
    }

    res
}


fn f(n:usize, p: &Vec<usize>) -> usize {
    for i in p.iter() {
        if n % *i == 0 {
            return *i;
        }
    }
    n
}


#[fastout]
fn main(){
    input!{
        n: usize, m: usize, 
        mut a: [usize; n],
    }
    
    let mut ans: Vec<bool> = vec![true; m+1];           /* 解答の管理 */
    let mut mem : HashSet<usize> = HashSet::new();      /* aの各要素の素因数の管理 */

    let x = *a.iter().max().unwrap();               
    let mut sfp: Vec<usize> = vec![0; x+1];             /* xまでの整数の最小の素因数を保存 */
    let prime = primes(x.sqrt());           /* 素数 */

    for i in 2..=x {
        sfp[i] = f(i, &prime);
    }
    
    for i in a.into_iter() {
        let mut tmp = i;
        while tmp != 1 {
            mem.insert(sfp[tmp]);
            tmp = tmp / sfp[tmp];
        }
    }

    for i in mem.into_iter() {
        for j in (i..=m).step_by(i) {
            ans[j] = false;
        }
    }


    println!("{}", ans.iter().skip(1).filter(|&&x| x).count());
    
    
    for i in 1..=m {
        if ans[i] {
            println!("{}", i);
        }
    }
    
}