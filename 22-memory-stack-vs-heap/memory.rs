fn increment_decrement(num: u8) {
    print_nums(num + 1, num - 1);
}

fn print_nums(x: u8, y: u8) {
    println!("{} {}", x, y);
}

increment_decrement(10);
