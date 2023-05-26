use proconio::input;

fn main(){
    input!{
        k: i64, s: i64, 
    }

    let mut cnt = 0;
    for x in 0..=k{
        for y in 0..=k{
            if s - x - y < 0{
                break;
            }else if s - x - y <= k {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);

}