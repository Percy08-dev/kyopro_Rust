use proconio::input;

fn main(){
    input!{
        n: usize, q: usize, 
        mut a: [u64; n], 
        query: [[usize; 3]; q], 
    }

    let mut shift:usize = 0;

    for i in query.clone(){
        if i[0] == 1{
            let x: usize = ((n + i[1] - shift - 1) % n + n) % n;
            let y: usize = ((n + i[2] - shift - 1) % n + n) % n;
            a.swap(x, y);
            // println!("@@@{} {}\t{} {}", i[1], shift, i[2], shift);
            // println!("{:?} {}", a, shift);
        }else if i[0] == 2{
            // println!("{:?} {}", a, shift);
            shift += 1;
        }else{
            let x:usize = ((n + i[1] - shift - 1) % n + n) % n;
            println!("{}", a[x]);
        }
    }

}