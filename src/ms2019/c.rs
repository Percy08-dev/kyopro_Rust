// 三井住友信託銀行 ２０１９コンテスト
use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        x: i32, 
    }

    if x <= 105 * (x/100) {
        println!("1");
    }else {
        println!("0");
    }
}