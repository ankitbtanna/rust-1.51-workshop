enum Color {
    Red,
    Yellow,
    Green
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
    }
}