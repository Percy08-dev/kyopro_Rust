use proconio::input;
use num::integer::lcm;

fn main(){
    input!{
        a:u128, b:u128, 
    }
    let res = lcm(a, b);

    if res > 1000000000000000000{
        println!("Large");
    }else{
        println!("{}", res);
    }
}