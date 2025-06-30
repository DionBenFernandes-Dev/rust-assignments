use chrono::{Local, Utc};

fn main() {
    let now_utc = Utc::now();
    println!("UTC Date & Time: {}", now_utc);

    let formatted_utc = now_utc.format("%Y:%m:%d %H:%M:%S");
    println!("Fommatted UTC Date & Time: {}", formatted_utc);

    let now_local = Local::now();
    println!("Local Date & Time: {}", now_local);
}
