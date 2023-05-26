use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        s: String,
    }
    let s:Vec<char> = s.chars().collect();
    let mut l:usize = 0;
    let mut r:usize = s.len()-1;
    let mut ans:String = "Yes".to_string();

    while l < r {
        if s[l] == s[r]{
            l += 1;
            r -= 1;
        }else if s[r] == 'a'{
            r -= 1;
        }else{
            ans = "No".to_string();
            break;
        }
    }

    println!("{}", ans);
}
