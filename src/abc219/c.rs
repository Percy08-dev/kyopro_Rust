use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        x: String, 
        n: usize, 
        s:[String; n], 
    }
    
    // let C: String = "abcdefghijklmnopqrstuvwxyz".to_string();
    let def_dic: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let new_dic: Vec<char> = x.chars().collect();
    
    /* 文字列を対応するインデックスの文字に変換し, sort. その後元に戻す */
    // 変換
    let mut mem: Vec<String> = Vec::new();
    for i in s.into_iter(){
        let mut tmp:Vec<char> = Vec::new();
        for j in i.chars().into_iter(){
            let a = new_dic.iter().position(|&x| x == j).unwrap();
            tmp.push(def_dic[a]);
        }
        mem.push(tmp.iter().collect());
    }

    // 並べ替え
    mem.sort_unstable();

    // 戻す
    let mut ans: Vec<String> = Vec::new();
    for i in mem.into_iter(){
        let mut tmp:Vec<char> = Vec::new();
        for j in i.chars().into_iter(){
            let a = def_dic.iter().position(|&x| x == j).unwrap();
            tmp.push(new_dic[a]);
        }
        ans.push(tmp.iter().collect());
    }

    for i in ans.into_iter(){
        println!("{}", i);
    }

}