use proconio::input;

fn main(){
    input!{
        n: usize, 
        a: [i64; n], 
    }
    /* 最適化してコンパイルすると, 型のoverflow判定が無くなる? */
    let m: i64 = 1_000_000_007;
    let mut sum: i64 = 0;
    let mut res: i64 = 0;

    for i in a.clone().into_iter(){
        sum += i;
        sum %= m;
    }

    for i in a.into_iter(){
        sum -= i;
        if sum < 0{
            sum += m;
        }
        res += sum * i;
        res %= m;
    }

    println!("{}", res);
}

