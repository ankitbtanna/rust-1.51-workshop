fn main() {
    let result = divide(10, 20);

    println!("{}", result);
}

fn divide(x: i32, y: u16) -> f64 {
    return (x as f64) / (y as f64);
}