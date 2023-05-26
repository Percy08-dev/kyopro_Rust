use itertools::Itertools;
use proconio::input;

fn main(){
    input!{
        h: usize, w:usize, 
        board: [[i64; w];h ], 
    }

    /* 横に圧縮 */
    /* 縦に圧縮 */
    let mut yoko: Vec<i64> = (0..h).map(|_| 0).collect_vec();
    let mut tate: Vec<i64> = (0..w).map(|_| 0).collect_vec();
    
    for i in 0..h{
        for j in 0..w{
            yoko[i] += board[i][j];
            tate[j] += board[i][j];
        }
    }

    let mut res:Vec<Vec<i64>> = Vec::new();
    /* 圧縮から答えを求める */
    for i in 0..h{
        let mut tmp: Vec<i64> = Vec::new();
        for j in 0..w{
            tmp.push(yoko[i] + tate[j] - board[i][j]);
        }
        res.push(tmp);
    }

    
    for i in res{
        for j in i{
            print!("{} ", j);
        }
        println!(" ");
    }
    
}