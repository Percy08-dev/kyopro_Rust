use std::{collections::BTreeSet, hash::Hash};
use rustc_hash::FxHasher;
use proconio::fastout;

struct timer {
    start: f64, 
    now: f64, 
}

impl timer {
    fn new() -> timer{
        timer{
            start: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64(),
            now: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64(),
        }
    }

    fn update(&mut self) {
        self.now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64();
    }

    fn time_delta(&self) -> f64{
        self.now - self.start
    }
}

#[fastout]

fn main(){
    let mut roop = 0;
    let mut res:Vec<f64> = Vec::new();

    for _i in 0..100 {
        let mut h:BTreeSet<i32> = BTreeSet::new();
        let mut t = timer::new();
        for i in 0..1_000_000 {
            h.insert(i+roop);
        }
        t.update();
        res.push(t.time_delta());
        print!("{}", h.iter().last().unwrap());
        roop += 1;
    }
    println!();
    println!("ave time:{}", res.iter().sum::<f64>()/100.0);

    roop = 0;
    let mut res:Vec<f64> = Vec::new();
    for _i in 0..100 {
        let mut h:BTreeSet<i32> = BTreeSet::new();
        let mut Fx = FxHasher::default();
        h.hash(&mut Fx);
        let mut t = timer::new();
        for i in 0..1_000_000 {
            h.insert(i+roop);
        }
        t.update();
        res.push(t.time_delta());
        print!("{}", h.iter().last().unwrap());
        roop += 1;
    }
    println!();
    println!("ave time:{}", res.iter().sum::<f64>()/100.0);

}