
enum Shapes {
    Rectangle(f32,f32), // width, height
    Square(f32), // side
}

fn calculate_area(shape: Shapes) -> f32 {
    let ans = match shape {
        Shapes::Rectangle(a,b) => a * b,
        Shapes::Square(a) => a * a, 
    };

    return ans;
}

fn main() {
    let rect = Shapes::Rectangle(1.2,2.0);
    println!("Area of Rectangle: {}", calculate_area(rect));

    let square = Shapes::Square(2.0);
    println!("Area of Square: {}", calculate_area(square));
}
