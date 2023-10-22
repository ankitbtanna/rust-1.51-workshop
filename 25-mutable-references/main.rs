fn main() {
    let mut years: Vec<i32> = vec![2001, 2002, 2003, 2004];
    let mutable_years: &mut Vec<i32> = &mut years;

    let years2: &mut Vec<i32> = &mut years;
    let years3: &Vec<i32> = &years;

    years3.clear();
    years2.len();
}