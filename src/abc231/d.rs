use proconio::{input, fastout};
use petgraph::unionfind::UnionFind;

#[fastout]

fn main(){
    input!{
        n: usize, m: usize, 
        join: [(usize, usize); m], 
    }

    let mut union_find = UnionFind::new(n+1);
    let mut cnt: Vec<usize> = vec![0; n+1];
    let mut ans = true;

    for (i, j) in join.into_iter(){
        cnt[i] += 1;
        cnt[j] += 1;
        if cnt[i] > 2 || cnt[j] > 2 || !union_find.union(i, j){
            ans = false;
            break;
        }
    }


    if ans {
        println!("Yes");
    }else {
        println!("No");
    }

}