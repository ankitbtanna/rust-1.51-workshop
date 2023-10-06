use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let crash_reason = "server took a nap";
    panic!("I crashed {}!", crash_reason);
    println!("This will never get printed out!");
}