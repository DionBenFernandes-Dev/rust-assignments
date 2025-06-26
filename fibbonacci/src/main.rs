fn main() {
    let x = 5;
    println!("Fibbonacci position of {} is {}",x,fib(x));
}

fn fib (num: u32) -> u32 {
    let mut num1 = 0;
    let mut num2 = 1;

    if num == 0 {
        return num1;
    }
    if num == 1 {
        return num2;
    }

    for _ in 1..num {
        let temp = num2;
        num2 =  num1 + temp;
        num1 = temp;
    }
    return num2;
}