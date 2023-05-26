/* intro_heuristics_contest_b */
use proconio::{input, fastout};


const BASE:i32 = 1_000_000;

fn scoring(t: &[usize], c: &[i32], s: &[Vec<i32>]) -> Vec<i32> {
    let mut contest = vec![0; 26];
    let mut res = Vec::new();

    for (i, c_num) in t.iter().enumerate() {
        /* 前処理 */
        let day = i as i32 + 1;                 /* 日付 */
        let mem = if res.last().is_some() {     /* 前日のスコア */
            *res.last().unwrap()
        }else{
            0
        };
        contest[*c_num - 1] = day;                  /* コンテスト実施日の更新 */

        /* 値の更新 */
        let add = s[i][*c_num - 1];
        let sub =  c.iter().zip(contest.iter()).map(|(&x, &y)| x*(day - y)).sum::<i32>();
        res.push(mem + add - sub);
    }

    res
}


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


// #[fastout]
fn main(){
    /* 
    /* A */
    input!{
        d: usize, 
        c: [i32; 26], 
        s: [[i32; 26]; d], 
    }

    /* config */
    let time:timer = timer { 
            start: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), 
            now: 0 
        };

    /* init */



    for i in scoring(&t, &c, &s).iter() {
        println!("{}", i);
    }
    */


    let mut time:timer = timer::new();
    for i in 0..1_000_000_00 {
        if i % 1_000_000_0 == 0 {
            time.update();
            println!("{}", time.time_delta());
        }
    }
}