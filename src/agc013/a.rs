use proconio::{input, fastout};

fn f(n: usize, a:Vec<usize>) -> i32 {
    let mut ans = 1;
    let mut op:bool = false;
    let mut flag = false;

    for i in 1..n {
        if a[i] != a[i-1] {
            op = a[i] > a[i-1];
            break;
        }
    }


    for i in 1..n {
        if a[i] == a[i-1] {
            continue;
        }

        if flag {
            op = a[i] > a[i-1];
            ans += 1;
            flag = false;
        }

        if (a[i] >= a[i-1]) != op {
            flag = true;
        }
    }

    if flag {
        ans += 1;
    }
    ans
}


#[fastout]
fn main(){
    input!{
        n: usize, 
        a: [usize; n], 
    }
    
    let ans:i32 = if n == 1 {
        1
    }else{
        f(n, a)
    };

    println!("{}", ans);
}