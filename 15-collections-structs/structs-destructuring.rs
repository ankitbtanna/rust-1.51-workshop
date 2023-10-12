struct Point {
    x: i64,
    y: i64,
    z: i64
}

fn main() {
    let point: Point = Point { x: 0, y: 1, z: 2 };
    let point1: Point = new_point(3, 4, 5);
    let point2: Point = new_point2(6, 7, 8);

    let Point {x, y, z} = point;
    let Point {x, y: _, z} = point1;
    let Point {x, ..} = point2;
}

fn new_point(a: i64, b: i64, c: i64) -> Point {
    return Point { x: a, y: b, z: c };
}

fn new_point2(x: i64, y: i64, z: i64) -> Point {
    Point { x, y, z }
}