use proconio::{input, fastout};
use std::collections::{VecDeque, HashSet};
use num::pow;

fn f1(a: i64, n: i64)->i64{
    a * n
}

fn f2(n: i64)-> i64{
    let mut x = n;
    let mut digit: usize = 0;
    if n/10 == 0 || n%10==0{
        return n;
    }
    while x != 0 {
        x /= 10;
        digit += 1;
    }
    n/10 + (n%10) * pow(10, digit-1)
}

fn f3(n: i64)->i64{
    let mut res = 1;
    while n > res {
        res *= 10;
    }
    res
}

#[fastout]

fn main(){
    input!{
        a: i64, n: i64,
    }

    let upper = f3(n);
    let mut used: HashSet<i64> = HashSet::new();
    let mut que: VecDeque<(i64, i64)> = VecDeque::new();
    let mut tmp  = (1, 0);      // (num, cnt)
    que.push_back(tmp);

    while tmp.0 != n{
        if que.is_empty(){
            tmp = (-1, -1);
            break;
        }
        tmp = que.pop_front().unwrap();
        if !used.get(&tmp.0).is_none(){
            continue;
        }
        let x = f1(a, tmp.0);
        if x < upper{
            que.push_back((x, tmp.1 + 1));
        }

        let x = f2(tmp.0);
        if x != tmp.0{
            que.push_back((x, tmp.1 + 1));
        }
        used.insert(tmp.0);
    }
    

    println!("{}", tmp.1);
}