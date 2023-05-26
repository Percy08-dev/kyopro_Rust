use proconio::input;

fn main(){
    input!{
        _n: usize, s: String, 
    }

    let mut left = 0;
    let mut abs_left: usize = 0;
    let mut right = 0;

    for i in s.chars().into_iter(){
        if i == ')' && left != 0{
            right += 1;
            left -= 1;
        }else if i == '(' {
            left += 1;
            right -= 1;
        }else{
            abs_left += 1;
        }
    }

    // println!("left:{}\nright:{}\nabs_left:{}", left, right, abs_left);
    let x = "(".repeat(abs_left);
    let y = if right < 0{
        ")".repeat(-right as usize)
    }else{
        String::from(" ")
    };
    
    println!("{}{}{}", x, s, y);
}