use num::pow;
use proconio::input;
use std::cmp::max;

fn main(){
    input!{
        n: usize, m:usize, 
        balls: [(usize, usize); m], 
        k: usize, 
        or_put: [(usize, usize); k], 
    }

    let mut res = 0;

    for i in 0..pow(2, k) as usize{
        let mut dish: Vec<usize>= vec![0; n];
        let mut cnt = 0;
        for j in 0..k{
            if i >> j & 1 == 1{
                dish[or_put[j].1 - 1] = 1;
            }else{
                dish[or_put[j].0 - 1] = 1;
            }
        }

        for i in balls.iter(){
            if dish[i.0 - 1] == 1 && dish[i.1 - 1] == 1{
                cnt += 1;
            }
        }

        res = max(res, cnt);
    }

    println!("{}", res);

}