use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, m: usize, 
    }

    let x: Vec<i32>;
    if n == 1 {
        x = (0..10).collect();
    }else if n == 2 {
        x = (10..100).collect();
    }else{
        x = (100..1000).collect();
    }

    let mut x: Vec<String> = x.iter().map(|x| x.to_string()).collect();
    let mut y: Vec<String> = Vec::new();

    for _i in 0..m{
        input!{s: usize, c: char}
        while !x.is_empty() {
            let tmp = x.pop().unwrap();
            if tmp.chars().nth(s-1).unwrap() == c{
                y.push(tmp);
            }
        }
        x = y.clone();
        y.clear();
    }

    if x.is_empty() {
        println!("-1");
    }else{
        let x = x.iter().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        println!("{}", x.iter().min().unwrap());
    }

}