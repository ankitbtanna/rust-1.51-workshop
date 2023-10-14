enum Color {
    Red,
    Yellow,
    Green,
    Custom(u8, u8, u8)
}

fn main() {
    let go: Color = Color::Green;
    let caution: Color = Color::Yellow;
    let stop: Color = Color::Red;
    let purple = Color::Custom { red: 128, green: 0, blue: 128 };
    let brown = Color::Custom(128, 64, 0);
}