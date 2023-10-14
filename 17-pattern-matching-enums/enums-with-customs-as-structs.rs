enum Color {
    Red,
    Yellow,
    Green,
    Custom { red: u8, green: u8, blue: u8 }
}

fn main() {
    let go: Color = Color::Green;
    let caution: Color = Color::Yellow;
    let stop: Color = Color::Red;
    let purple = Color::Custom { red: 128, green: 0, blue: 128 };
}