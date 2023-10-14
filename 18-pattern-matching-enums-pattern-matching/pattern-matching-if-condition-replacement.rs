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
        Color::Green => {
            "The color is Green!"
        }
        Color::Custom { red, green, blue } => {
            format!("The color is Custom with RGB values {}, {}, {}!", red, green, blue)
        }
    };
}