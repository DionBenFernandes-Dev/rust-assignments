struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32{
        return self.height * self.width;
    }

    fn perimeter(&self) -> u32{
        return 2 * (self.height + self.width);
    }

    fn debug() -> u32 {
        return 1;
    }
}

fn main() {
    let rect = Rect{
        width: 20,
        height: 10,
    };

    println!("Area of rect is {}",rect.area());
    println!("Perimeter of rect is {}",rect.perimeter());
    println!("Debug is {}",Rect::debug());
}
