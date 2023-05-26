use proconio::{input, fastout, source::line::LineSource};
use std::collections::HashSet;
use std::io::{stdout, Write, stdin, BufReader};
use std::iter::FromIterator;
use std::process::exit;


fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string().parse::<usize>().unwrap()
}


// #[fastout]
fn main(){
    let n = read();

    let mut nums: HashSet<usize> = HashSet::from_iter(1..=(2*n+1));

    while !nums.is_empty() {
        let tmp = *nums.iter().next().unwrap();
        println!("{}", nums.take(&tmp).unwrap());
        stdout().flush().ok();

        let tmp = read();
        if tmp == 0 {
            break;
        }
        nums.remove(&tmp);
    }
}