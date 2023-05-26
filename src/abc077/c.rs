use proconio::input;
use std::cmp::Ordering;


fn main(){
    input!{
        n: usize, 
        mut a: [usize; n], 
        mut b: [usize; n], 
        mut c: [usize; n], 
    }
    a.sort_unstable();
    b.sort_unstable();
    c.sort_unstable();

    let mut res = 0;
    
    for j in b.iter(){
        let i = a.lower_bound(j);
        let k = n - c.upper_bound(j);
        res += i * k;
    }



    println!("{}", res);
}

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
