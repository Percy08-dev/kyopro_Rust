use itertools::Itertools;
use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, 
        s: String, 
        w: [usize; n], 
    }

    let s = s.chars().map(|c| c == '1').collect_vec();
    let s = s.iter().zip(w.iter()).sorted_by(|&x, &y| x.1.cmp(y.1)).map(|(&x, _y)| x).collect_vec();

    let mut cn = Vec::new();
    let mut tmp = (1, s[0]);
    for i in 1..n {
        if s[i] == tmp.1 {
            tmp.0 += 1;
        }else {
            cn.push(tmp);
            tmp.0 = 1;
            tmp.1 = !tmp.1;
        }
    }
    cn.push(tmp);

    let mut ans = cn.iter().filter(|&&x| x.1).map(|&(x, _y)| x).sum::<i32>();
    let mut rg = ans;

    for &(n, b) in cn.iter() {
        if b == false {
            rg += n;
        }else {
            rg -= n;
        }
        ans = ans.max(rg);
    }

    println!("{}", ans);
}