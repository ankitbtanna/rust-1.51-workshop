fn main() {
    let point: (i64, i64, i64) = (0, 1, 2);

    let (x, y, z) = point;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    let (a, b, _) = point;

    println!("a: {}", a);
    println!("b: {}", b);

    let (d, _, f) = point;
    println!("d: {}", d);
    println!("f: {}", f);
}