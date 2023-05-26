use proconio::{input, fastout};

#[fastout]
fn main(){
    input!{
        n: usize, 
    }

    let mut s: Vec<Vec<char>> = Vec::with_capacity(n);

    for _i in 0..n {
        input! {tmp:String}
        s.push(tmp.chars().collect::<Vec<char>>());
    }

    let mut ans = "No".to_string(); 

    // 横
    for y in 0..n {
        let mut tmp: u32 = 0;
        for x in 0..n {
            if s[y][x] == '#' {
                tmp = ( (tmp << 1) + 1) % 64; 
            }else{
                tmp = (tmp << 1) % 64;
            }

            if tmp.count_ones() == 4 {
                ans = "Yes".to_string();
                break;
            }
        }

        if ans == "Yes" {
            break;
        }
    }

    // 縦
    if ans == "No" {
        for x in 0..n {
            let mut tmp: u32 = 0;
            for y in 0..n {
                if s[y][x] == '#' {
                    tmp = ( (tmp << 1) + 1) % 64; 
                }else{
                    tmp = (tmp << 1) % 64;
                }

                if tmp.count_ones() == 4 {
                    ans = "Yes".to_string();
                    break;
                }
            }

            if ans == "Yes" {
                break;
            }
        }
    }

    // 斜め
    if ans == "No" {
        for y in 0..n {
            for x in 0..n {
                // 左下～右上
                let mut tmp: u32 = 0;
                let mut cnt = 0;
                for i in -6..7 {
                    let xx = if (x as i32) - i < 0 || (x as i32) - i >= n as i32 {
                        continue
                    }else{
                        ((x as i32) - i) as usize
                    };

                    let yy = if (y as i32) + i < 0 || (y as i32) + i >= n as i32 {
                        continue
                    }else{
                        ((y as i32) + i) as usize
                    };
                    
                    if s[yy][xx] == '#' {
                        tmp = ( (tmp << 1) + 1) % 64; 
                    }else{
                        tmp = (tmp << 1) % 64;
                    }
                    if tmp.count_ones() == 4 {
                        ans = "Yes".to_string();
                        
                    }
                    cnt += 1;
                }

                if cnt >= 6 && ans == "Yes" {
                    break;
                }else if cnt < 6 {
                    ans = "No".to_string();
                }
                
                // 右下
                let mut tmp: u32 = 0;
                let mut cnt = 0;
                for i in 0..6 {
                    let xx = if x + i < n {
                        x + i
                    }else{
                        break
                    };

                    let yy = if y + i < n {
                        y + i
                    }else{
                        break
                    };

                    if s[yy][xx] == '#' {
                        tmp = ( (tmp << 1) + 1) % 64; 
                    }else{
                        tmp = (tmp << 1) % 64;
                    }
                    if tmp.count_ones() == 4 {
                        ans = "Yes".to_string();
                    }
                    cnt += 1;
                }

                if cnt >= 6 && ans == "Yes" {
                    break;
                }else if cnt < 6 {
                    ans = "No".to_string();
                }
            }

            if ans == "Yes" {
                break;
            }
        }
    }

    println!("{}", ans);


}