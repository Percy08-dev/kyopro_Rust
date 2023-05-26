use proconio::input;

fn main(){
    input!{
        n: u32,
    }

    for i in (0..(1<<n)).rev(){
        let mut stuck: Vec<char> = Vec::new();
        let mut mem = 0;
        let mut flag:bool = true;

        for j in (0..n).rev(){
            if (i as usize >> j) & 1 == 1{
                stuck.push('(');
                mem += 1;
            }else{
                stuck.push(')');
                mem -= 1;
            }
            if mem < 0{
                flag = false;
                break;
            }
        }

        if mem != 0{
            flag = false;
        }

        if flag{
            let res:String = stuck.iter().collect();
            println!("{}", res);
        }
    }
}