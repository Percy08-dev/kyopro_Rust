use proconio::input;


fn main(){
    input!{
        n: usize, x:usize, 
    }
    let mut balls: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n{
        input! {L:usize, mut tmp:[usize; L]}
        balls.push(tmp);
    }

    /* solve */
    let mut old: Vec<usize> = Vec::new();
    let mut new: Vec<usize> = Vec::new();
    let mut cnt = 0;
    old.push(x);

    for i in balls{
        new.clear();
        for j in i{
            // for a in 0..old.len(){
            for a in old.iter(){
                if a % j == 0 && *a != 0 { 
                    new.push(a/j);
                }
            }
        }
        /* 初期化(もっといい方法あるだろ) */
        old.clear();
        for i in new.iter(){
            old.push(*i);
        }
        // println!("{:?}", old);
    }
    
    for i in old{
        if i == 1{
            cnt += 1;
        }
    }
    println!("{}", cnt);
}