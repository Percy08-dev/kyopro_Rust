use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        s: String, 
    }

    let mut point = (false, false, false, false); /* N, S, E, W */

    for i in s.chars().into_iter(){
        if i == 'N' {
            point.0 = true;
        }else if i == 'S' {
            point.1 = true;
        }else if i == 'E' {
            point.2 = true;
        }else{
            point.3 = true;
        }
    }

    if point.0 == point.1 && point.2 == point.3 {
        println!("Yes");
    }else{
        println!("No");
    }

}