fn main() {
    let years: Vec<i32> = vec![2001, 2002, 2003, 2004];
    let slice = &nums[0..2];
    let name = "hello world!";
    let name_slice = &name[0..2];

    let full_name = name.as_str();
    let full_years = years.as_slice();
}