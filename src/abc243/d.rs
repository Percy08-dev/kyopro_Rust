use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, mut x:usize, 
        s: String, 
    }

    /* 圧縮  */
    let mut new_s: Vec<char> = Vec::with_capacity(n);
    
    for i in s.chars() {
        if i == 'U' {
            if new_s.is_empty() {
                x /= 2;
            }else{
                new_s.pop();
            }
        }else {
            new_s.push(i);
        }
    }

    for i in new_s.into_iter() {
        if i == 'L' {
            x *= 2;
        }else {
            x = (x*2) + 1;
        }
    }

    println!("{}", x);

}