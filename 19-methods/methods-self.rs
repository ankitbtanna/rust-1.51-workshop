enum Color {
    Red,
    Yellow,
    Green,
    Custom { red: u8, green: u8, blue: u8 }
}

impl Color {
    // Associated function that creates a custom Color
    fn custom_color(r: u8, g: u8, b: u8) -> Self {
        Color::Custom { red: r, green: g, blue: b }
    }

    // Method that prints information about the Color
    fn describe(&self) {
        match self {
            Self::Red => {
                println!("The color is Red!");
            }
            Self::Yellow => {
                println!("The color is Yellow!");
            }
            Self::Green => {
                println!("The color is Green!");
            }
            Self::Custom { red, green, blue } => {
                println!("Custom color: R={}, G={}, B={}", red, green, blue);
            }
        }
    }
}

fn main() {
    let red_color = Color::Red;
    let custom_color = Color::custom_color(255, 0, 0);

    red_color.describe();
    custom_color.describe();
}
