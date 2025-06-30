fn main() {
    let s = String::from("Mathew");
    let ans = find_first_a(&s);

    match ans {
        CustomOption::Some(value) => println!("a is found in {} postion in string {}.",value,s),
        CustomOption::None => println!("a is not found in string {}",s),
    }
}

pub enum CustomOption {
    Some(i32),
    None,
}

fn find_first_a(s: &String) -> CustomOption {
    for (i,c) in s.chars().enumerate() {
        if c == 'a' {
            return CustomOption::Some(i as i32);
        }
    }

    return CustomOption::None;
}
