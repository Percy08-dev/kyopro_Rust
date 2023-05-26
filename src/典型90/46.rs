use proconio::input;
use std::collections::HashMap;

fn main(){
    input!{
        n: usize, 
        a: [usize; n], 
        b: [usize; n], 
        c: [usize; n], 
    }
    
    let mut col_a: HashMap<usize, usize> = HashMap::new();
    let mut col_b: HashMap<usize, usize> = HashMap::new();
    let mut col_c: HashMap<usize, usize> = HashMap::new();

    for i in 0..n{
        *col_a.entry(a[i] % 46).or_insert(1) += 1;
        *col_b.entry(b[i] % 46).or_insert(1) += 1;
        *col_c.entry(c[i] % 46).or_insert(1) += 1;
    }

    let mut res:usize = 0;
    
    for (i, x) in col_a.iter(){
        for (j, y) in col_b.iter(){
            for (k, z) in col_c.iter(){
                if (i+j+k) % 46 == 0{
                    res += (x-1)*(y-1)*(z-1);
                }
            }
        }
    }

    println!("{}", res);
}