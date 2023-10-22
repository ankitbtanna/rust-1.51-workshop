fn get_years() -> Vec<i32> {
    let years = vec![2001, 2002, 2003, 2004]; // alloc()

    return years; // transfer the ownership of years to main()
}

fn main() {
    let years = get_years();
}