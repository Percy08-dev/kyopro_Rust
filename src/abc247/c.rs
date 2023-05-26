use proconio::{input, fastout};

fn r(lim:usize, depth:usize, now:&mut Vec<usize>) {
    if lim > depth {
        now.push(depth+1);
        now.extend(now.clone().iter().take(now.len()-1));
        r(lim, depth+1, now);
    }
}

#[fastout]
fn main(){
    input!{
        n: usize, 
    }

    let mut v:Vec<usize> = vec![1];
    r(n, 1, &mut v);

    for i in v.into_iter() {
        print!("{} ", i);
    }
    println!();

}