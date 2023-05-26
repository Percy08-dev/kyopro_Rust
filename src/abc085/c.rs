use proconio::input;

fn main(){
    input!{
        n: i32, y: i32, 
    }
    
    let mut res = (-1, -1, -1);

    for i in 0..=n{
        for j in 0..=(n-i){
            let tmp = y - i*10000 - j*5000;
            if tmp >= 0 && tmp % 1000 == 0 && tmp / 1000 == n - i - j{
                res = (i, j, tmp / 1000);
            } 
        }
    }

    println!("{} {} {}", res.0, res.1, res.2);
}