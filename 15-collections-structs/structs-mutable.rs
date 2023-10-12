struct Point {
    x: i64,
    y: i64,
    z: i64
}

fn main() {
    let mut point: Point = Point { x: 0, y: 1, z: 2 };

    point.x = point.x + 1;

    println!("Point x: {}", point.x);
}
