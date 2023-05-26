use proconio::{input, fastout};
use std::collections::HashSet;
use std::iter::FromIterator;

fn read() -> String {
    let mut s = String::new(); // バッファを確保
    std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
    s.trim().to_owned()
}

#[fastout]
fn main(){
    /* 
    input!{
        s: String, 
        t: String,
    }
    */
    let s = read();
    let t = read();

    // let x:HashSet<(char, char, char)> = HashSet::from([('A', 'B', 'C'), ('B', 'C', 'A'), ('C', 'A', 'B')]);
    let x:HashSet<String> = HashSet::from_iter(vec!["R G B", "G B R", "B R G"].iter().map(|x| x.to_string()));

    if x.get(&s).is_some() == x.get(&t).is_some() {
        println!("Yes");
    }else{
        println!("No");
    }
}