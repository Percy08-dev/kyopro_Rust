use std::usize;

use proconio::input;

fn main(){
    input!{
        n: usize, 
        stat: [[usize; 2]; n], 
        q: usize, 
        query: [[usize; 2]; q], 
    }

    /* 累積和 */
    let mut sum_stat_a: Vec<usize> = vec![0];
    let mut sum_stat_b: Vec<usize> = vec![0];
    let mut c1: usize = 0;
    let mut c2: usize = 0;
    
    for i in 0..n{
        if stat[i][0] == 1{
            c1 += stat[i][1];
        }else{
            c2 += stat[i][1];
        }
        sum_stat_a.push(c1);
        sum_stat_b.push(c2);
    }

    for i in query.iter(){
        let a = sum_stat_a[i[1]] - sum_stat_a[i[0]-1];
        let b = sum_stat_b[i[1]] - sum_stat_b[i[0]-1];
        println!("{} {}", a, b);
        // println!("{} {}", i[1]-1, i[0]-1);
    }   

    // println!("{:#?}", sum_stat_a);
    // println!("{:?}", sum_stat_a);
    // println!("{:?}", sum_stat_b);

}