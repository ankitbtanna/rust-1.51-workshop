fn print_years(years: Vec<i32>) {
    for year in years.iter() {
        println!("{}", year);
    }
    return years;
}

fn main() {
    let years = vec![2001, 2002, 2003, 2004]; // alloc()

    let years2 = print_years(years); // transfer the ownership of years to print_years()
    let years3 = print_years(years2);
}