use proconio::input;

fn split_cnt(lb:usize, a:Vec<usize>)->usize{
    let mut l: usize = 0;
    let mut cnt: usize = 0;

    for i in 0..a.len(){
        l += a[i];
        if l >= lb{
            l = 0;
            cnt += 1;
        }
    }
    
    cnt
}

fn bisect(mut right:usize, mut left:usize, a:Vec<usize>, k:usize)->usize{
    let mut mid: usize;

    while right - left > 1 {
        mid = left + (right - left)/2;
        if split_cnt(mid, a.clone()) > k{
            left = mid;
        }else{
            right = mid;
        }
    }

    left
}

fn main(){
    input!{
        n: usize, l: usize, k: usize, a: [usize; n], 
    }

    /* 良くなさそう */
    let mut tmp: Vec<usize> = Vec::new();
    tmp.push(0);
    for i in 0..n{
        tmp.push(a[i]);
    }
    tmp.push(l);

    let mut cut: Vec<usize> = Vec::new();
    for i in 0..tmp.len()-1{
        cut.push(tmp[i+1]-tmp[i]);
    }
    
    let res = bisect(l+1, 0, cut, k);
    println!("{}", res);
}