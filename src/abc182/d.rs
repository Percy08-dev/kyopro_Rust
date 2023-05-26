use proconio::input;
use std::cmp::max;

fn main(){
    input!{
        n: usize, 
        a: [i64; n], 
    }

    let mut sums: Vec<i64> = Vec::with_capacity(n);
    let mut maxs: Vec<i64> = Vec::with_capacity(n);
    let mut res = max(a[0], 0);
    let mut line_sum = a[0];
    sums.push(a[0]);
    maxs.push(res);

    for (i, j) in a.into_iter().skip(1).enumerate(){
        line_sum += j;
        sums.push(sums[i] + line_sum);
        maxs.push(max(maxs[i], line_sum));
    }

    for i in 1..n{
        res = max(res, sums[i-1] + maxs[i]);
    }

    println!("{}", res);
    // println!("{:?}", sums);
    // println!("{:?}", maxs);
}