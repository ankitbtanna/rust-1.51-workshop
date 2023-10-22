fn print_years(years: &Vec<i32>) {
    for year in years.iter() {
        println!("{}", year);
    }
}

fn main() {
    let years = vec![2001, 2002, 2003, 2004]; // alloc()

    print_years(&years); // transfer the ownership of years to print_years()
    print_years(&years);
}