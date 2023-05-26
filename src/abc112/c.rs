use proconio::input;

fn main(){
    input!{
        n: usize, 
        ph: [(i32, i32, i32); n], 
    }

    // let mem = ph.iter().filter(|(_, _, h)| *h!=0).next().unwrap();
    let mem = ph.iter().find(|(_, _, h)| *h!=0).unwrap();

    for i in 0..=100{
        for j in 0..=100{
            let tmp_H = mem.2 + (i-mem.0).abs() + (j-mem.1).abs();
            if ph.clone().iter().all(|(x, y, h)| *h == (tmp_H - (i-x).abs() - (j-y).abs()).max(0)){
                println!("{} {} {}", i, j, tmp_H);
                break;
            }
            
        }
    }
}
