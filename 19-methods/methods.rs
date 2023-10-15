enum Color {
    Red,
    Yellow,
    Green,
    Custom { red: u8, green: u8, blue: u8 }
}

impl Color {
    fn rgb(color: Color) -> (u8, u8, u8) {
        match color {
            Color::Red => (255, 0, 0),
            Color::Yellow => (255, 255, 0),
            Color::Green => (0, 255, 0),
            Color::Custom { red, green, blue } => (red, green, blue)
        }
    }
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color::Custom { red: r, green: g, blue: b }
    }
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

    let color = Color::new(255, 0, 0);
    let rgb = Color::rgb(color);

    println!("{} {} {}", rgb.0, rgb.1, rgb.2);
}