use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        s: String, 
    }

    let s: Vec<char> = s.chars().collect();

    let mut i = 0;
    let mut cnt = 0;
    let mut bef = String::new();
    let mut tmp = String::new();

    while i < s.len() {
        tmp.push(s[i]);
        if bef != tmp {
            bef = tmp;
            tmp = String::new();
            cnt += 1;
        }
        i += 1;
    }

    println!("{}", cnt);
}