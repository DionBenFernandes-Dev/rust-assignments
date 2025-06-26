#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}


fn main() {
    let current_direction = Direction::North;
    println!("current direction: {:?}", current_direction);
    //let new_direction = move_current_direction(current_direction);
    //println!("new direction: {}", new_direction);
}

// fn move_current_direction(direction: Direction) -> Direction{
//     let direction = Direction::West;
//     return direction;
// }
