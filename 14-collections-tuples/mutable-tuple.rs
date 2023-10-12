fn main() {
    let mut point: (i64, i64, i64) = (0, 1, 2);

    point.0 = point.0 + 1;
    point.1 = point.1 + 1;
    point.2 = point.2 + 1;

    println!("Point 0: {}", point.0);
    println!("Point 1: {}", point.1);
    println!("Point 2: {}", point.2);
}