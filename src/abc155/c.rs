use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main(){
    input!{
        n: usize, 
        s: [String; n], 
    }
    // カウント
    let mut counter: HashMap<String, i32> = HashMap::new();

    for i in s.into_iter(){
        *counter.entry(i).or_insert(0) += 1;
    }

    // HashMapからvecに変換
    let mut vcounter: Vec<(String, i32)> = Vec::with_capacity(counter.len());

    for i in counter.into_iter(){
        vcounter.push(i);
    }
    // 出現数でソート
    vcounter.sort_unstable_by(|x, y| x.1.cmp(&y.1));

    let mut ans: Vec<String> = Vec::new();
    let mem = vcounter.pop().unwrap();
    ans.push(mem.0);
    
    // 個数が異なる物が出るまで追加
    while !vcounter.is_empty() {
        let tmp = vcounter.pop().unwrap();
        if mem.1 == tmp.1{
            ans.push(tmp.0);
        }else{
            break;
        }
    }

    // 出力
    ans.sort_unstable();
    for i in ans.into_iter(){
        println!("{}", i);
    }
    

}