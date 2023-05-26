use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        l: usize, r: usize, 
        s: String, 
    }

    let l = l - 1;
    let r = r - 1;

    let mut ans:String = s.chars().into_iter().take(l).collect();
    let s:Vec<char> = s.chars().into_iter().collect();

    for i in (l..=r).rev() {
        ans.push(s[i]);
    }
    
    ans += &s.into_iter().skip(r+1).collect::<String>();

    println!("{}", ans);

}