use proconio::{input, fastout};
use std::iter::FromIterator;

#[fastout]
fn main(){
    input!{
        n: usize, 
        mut b: [(usize, usize); n], 
    }

    /* 解答 */
    let mut ans = vec![0; n + 1];
    let start:Vec<usize> = Vec::from_iter(b.iter().map(|&x| x.0));
    let end:Vec<usize> = Vec::from_iter(b.iter().map(|&x| x.0 + x.1));

    /* startとendにマーカーを付けて1つにする */
    let mut q:Vec<(usize, char)> = Vec::from_iter(start.iter().map(|&x| (x, 's')));
    q.extend(end.iter().map(|&x| (x, 'e')));
    q.sort_unstable();

    let mut cnt = 0;
    let mut day = 0;

    for i in q.into_iter() {
        ans[cnt] += i.0 - day;
        day = i.0;
        if i.1 == 's' {
            cnt += 1;
        }else{
            cnt -= 1;
        }
    }


    for i in ans.iter().skip(1) {
        print!("{} ", i);
    }
    println!();

}