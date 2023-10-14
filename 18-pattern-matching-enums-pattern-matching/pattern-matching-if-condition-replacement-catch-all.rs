enum Color {
    Red,
    Yellow,
    Green,
    Custom { red: u8, green: u8, blue: u8 }
}

fn main() {
    let current_color: Color = Color::Green;

    let color_description = match current_color {
        Color::Red => {
            "The color is Red!"
        }
        Color::Yellow => {
            "The color is Yellow!"
        }
        _ => {
            "The color is a different color!"
        }
    };
}