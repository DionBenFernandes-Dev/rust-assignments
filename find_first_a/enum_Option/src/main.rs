fn main() {
    let ans = find_first_a(String::from("Mathew"));

    match ans {
        Some(value)=>println!("a is found in position {}.", value),
        None=>println!("a is not found"),
    };
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }

    return None;
}