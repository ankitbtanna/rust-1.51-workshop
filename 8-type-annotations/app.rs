fn main() {
    let x = 1.1;
    let y = 2.2;

    let answer = multiply_both(x, y);

    println!("x times y is {}!", answer);   
}

fn multiply_both(x: f64, y: f64) -> f64 {
    return x*y;
}