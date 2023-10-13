fn main() {
    let mut years: [i32; 3] = [2021, 2022, 2023];
    let first_year = years[0];
    let [_, second_year, third_year] = years;
    let x = 1;
    let year_by_index = years[x];

    // years[3] <-- compile time panic
    // years[x] <-- run time panic if x > 2

    years[2] = 2024;

    for year in years.iter() {
        println!("Year: {}", year);
    }
}