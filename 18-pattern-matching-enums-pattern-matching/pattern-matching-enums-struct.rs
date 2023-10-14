enum Color {
    Red,
    Yellow,
    Green,
    Custom { red: u8, green: u8, blue: u8 }
}

fn main() {
    let current_color: Color = Color::Green;

    match current_color {
        Color::Red => {
            println!("The color is Red!");
        }
        Color::Yellow => {
            println!("The color is Yellow!");
        }
        Color::Green => {
            println!("The color is Green!");
        }
        Color::Custom { red, green, blue } => {
            println!("The color is Custom with RGB values {}, {}, {}!", red, green, blue);
        }
    }
}