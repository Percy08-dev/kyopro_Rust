use proconio::input;

fn main(){
    input!{
        n: usize, 
        mut p_red: [(i64, i64); n], 
        mut p_blue: [(i64, i64); n]
    }
    
    p_red.sort_unstable_by_key(|&(_,y)| y);
    p_blue.sort_unstable_by_key(|&(x, _)| x);
    let mut bools = vec![true; n];
    let mut cnt = 0;

    // println!("{:?}", p_red);
    // println!("{:?}", p_blue);

    for (x1, y1) in p_blue.into_iter(){
        for (i, (x2, y2)) in p_red.iter().rev().enumerate(){
            if bools[i] && x2 < &x1 && y2 < &y1{
                cnt += 1;
                bools[i] = false;
                break;
            }
        }
    }
    

    println!("{}", cnt);
}