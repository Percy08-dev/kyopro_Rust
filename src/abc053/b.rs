use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        s: String, 
    }

    let s:Vec<char> = s.chars().collect();
    let mut start = -1;
    let mut end = -1;

    for i in 0..s.len(){
        if s[i] == 'A' && start == -1{
            start = i as i32;
        }
        if s[s.len()-1-i] == 'Z' && end == -1{
            end = (s.len()-1-i) as i32;
        }
        if start != -1 && end != -1{
            break;
        }
    }


    println!("{}", end - start + 1);

}