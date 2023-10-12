struct Point {
    x: i64,
    y: i64,
    z: i64
}

fn main() {
    let point: Point = Point { x: 0, y: 1, z: 2 };
    let point1: Point = new_point(3, 4, 5);
    let point2: Point = new_point2(6, 7, 8);

    println!("Point x: {}", point.x);
    println!("Point y: {}", point.y);
    println!("Point z: {}", point.z);

    println!("Point x: {}", point1.x);
    println!("Point y: {}", point1.y);
    println!("Point z: {}", point1.z);

    println!("Point x: {}", point2.x);
    println!("Point y: {}", point2.y);
    println!("Point z: {}", point2.z);
}

fn new_point(a: i64, b: i64, c: i64) -> Point {
    return Point { x: a, y: b, z: c };
}

fn new_point2(x: i64, y: i64, z: i64) -> Point {
    Point { x, y, z }
}