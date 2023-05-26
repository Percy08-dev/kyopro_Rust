use proconio::{input, fastout};

// #[fastout]
fn main(){
    input!{
        n: usize, 
        s: [String; n], 
    }
    let s:Vec<Vec<char>> = s.into_iter().map(|x| x.chars().collect()).collect();

    
    let mut num_timer: Vec<Vec<usize>> = vec![Vec::with_capacity(10); 10];
    // let mut num_timer: Vec<Vec<Vec<usize>>> = vec![vec![Vec::with_capacity(10); 10]; 10];
    
    for i in 0..10 {
        for row in s.iter() {
            let x = row[i] as usize - 48;
            num_timer[x].push(i);
        }
    }
    
    // calc
    for i in 0..10 {
        for j in 0..n-1 {
            for k in j+1..n{
                if num_timer[i][j] == num_timer[i][k] {
                    num_timer[i][k] = num_timer[i][j] + 10;
                }
            }
        }
        num_timer[i].sort_unstable();
    }
    println!("{}", num_timer.iter().map(|x| x.last().unwrap()).min().unwrap());
}
