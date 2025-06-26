fn main() {
    let s = String::from("Hello, world!");
    let l = get_string_length(&s);
    println!("Length of {} is {}", s, l);
}

fn get_string_length (s: &String) -> usize {
    return s.len();
    //return s.chars().count();
}
