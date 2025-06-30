use std::fs::read_to_string;

// Type 1 
// fn main() {
//     let filename = String::from("a.txt");
//     let ans = read_file_from_filename(filename);

//     match ans {
//         Ok(value) => println!("{}",value),
//         Err(err) =>  println!("{}", err),
//     }
// }

// fn read_file_from_filename(file_path: String) -> Result <String, String> {
//     let result = read_to_string(file_path);
//     match result {
//         Ok(data) => Ok(data as String),
//         Err(_err) => Err(String::from("File not found")),
//     }
// }


// Type 2
fn main(){
    let ans = read_to_string(String::from("a.txt")); // change to b.txt => "File not found"

    match ans {
        Ok(data) => println!("{}", data as String),
        Err(_err) => println!("File not found"),
    }
}

