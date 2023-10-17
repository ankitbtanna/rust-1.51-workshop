fn main() {
    let mut years: Vec<i32> = vec![2001, 2002, 2003, 2004, 2005];
    years.push(2006);
    println!("The number of years are {}", years.len());
    years.push(2007);
    println!("The number of years are {}", years.len());

    let number_of_years: usize = years.len(); // usize = u32 or u64 depending on the system
    
    let first_year_index: usize = 0;
    let first_year: i32 = years[first_year_index];
    println!("The first year is {}", first_year);

    let mut nums: [u32; 3] = [1, 2, 3];
    let mut nums_vec: Vec<u32> = vec![1, 2, 3];

    // nums.push(4); <-- this will panic

    for year in years.iter() {
        println!("The year is {}", year);
    }
}