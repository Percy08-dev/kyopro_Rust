use proconio::input;

fn main(){
    input!{
        n: usize, k: usize, 
        mut ab: [[usize; 2]; n], 
    }
    let mut res: usize = 0;
    let mut tmp: Vec<usize> = Vec::with_capacity(n*2);

    for i in ab{
        tmp.push(i[1]);
        tmp.push(i[0] - i[1]);
    }

    tmp.sort_by(|a, b| a.cmp(b).reverse());

    for i in 0..k{
        res += tmp[i];
    }

    println!("{}", res);


}