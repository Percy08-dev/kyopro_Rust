use std::vec;

use proconio::{input, fastout};

fn f(n: usize, m: usize) -> usize{
    let mut data: Vec<Vec<usize>> = vec![vec![0; m]; n];
    let a = n as i32;
    let b = m as i32;

    for x in 0..m as i32{
        for y in 0..n as i32{
            for xi in -1..2 {
                for yi in -1..2 {
                    if (x + xi < 0 || x + xi > b-1) || (y + yi < 0 || y + yi > a-1) {
                        continue;
                    }
                    data[(y + yi) as usize][(x + xi) as usize] = (data[(y + yi) as usize][(x + xi) as usize] + 1) % 2;
                }
            }
        }
    }

    let mut ans = 0;
    for i in data.into_iter() {
        ans +=  i.iter().sum::<usize>();
    }
    ans
}


#[fastout]
fn main(){
    input!{
        n: usize, m: usize, 
    }

    let ans;

    if n < 4 && m < 4 {
        ans = f(n, m);
    }else if n < 4 {
        if n % 2 == 1 {
            ans = (m - 2) * (n % 2);
        }else {
            ans = 0;
        }
    }else if m < 4 {
        if m % 2 == 1 {
            ans = (n - 2) * (m % 2);
        }else {
            ans = 0;
        }
    }else {
        ans = (n-2) * (m-2);
    }

    println!("{}", ans);
}