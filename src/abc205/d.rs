use proconio::{input, fastout};
use std::cmp::{Ordering, max};

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}


#[fastout]
fn main(){
    input!{
        n: usize, q: usize, 
        a:[usize; n], 
    }
    let mut dif:Vec<usize> = (1..n).into_iter().map(|i| a[i]-a[i-1]-1).collect();
    dif.insert(0, a[0]-1);

    let mut sums:Vec<usize> = vec![0];
    let mut bef = 0;
    for i in dif.iter() {
        sums.push(bef + *i);
        bef += i;
    }
    
    // println!("{:?}",sums);

    for _i in 0..q {
        input!{k:usize}
        let idx = sums.upper_bound(&k)-1;
        // println!("@{}", idx);
        if k - sums[idx] == 0 {
            if idx == 1 {
            println!("{}", k);
            }else{
                let x = sums.lower_bound(&k) - 2;
                println!("{}", a[x] + 1);
            }
        }else {
            let inc = k - sums[idx];
            println!("{}", a[idx-1] + inc);
        }
    }

}