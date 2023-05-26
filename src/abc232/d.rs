use proconio::input;
use std::cmp::max;

/* つながりが無い場合, 0になる為これでいい */
fn solver(h:usize, w:usize, c: Vec<String>) -> usize{
    let mut res:Vec<Vec<usize>> = Vec::new();

    /* 配列の初期化 */
    for _i in 0..=h{
        let mut mem:Vec<usize> = Vec::new();
        for _j in 0..=w{
            mem.push(0);
        }
        res.push(mem);
    }

    for i in (0..h).rev(){
        for j in (0..w).rev(){
            if c[i].chars().nth(j).unwrap() == '.'{
                res[i][j] = max(res[i+1][j], res[i][j+1]) + 1;
            }
        }
    }

    res[0][0]
}

fn main(){
    input!{
        h: usize, w: usize, 
        c: [String; h],
    }

    let res: usize = solver(h, w, c);

    println!("{}", res);

}