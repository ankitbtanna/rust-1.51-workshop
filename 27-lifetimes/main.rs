struct Releases<'y> {
    years: &'y [i64],
    eighties: &'y [i64],
    nineties: &'y [i64],
}

fn main() {
    let years: Vec<i64> = vec![1988, 1989, 1990, 1991, 2004, 2006];

    let (eighties, nineties) = {
        let all_years: Vec<i64> = vec![1988, 1989, 1990, 1991, 2004, 2006];
        let releases = jazz_releases(&all_years);
        (releases.eighties, releases.nineties)
    };

    println!("eighties: {}", eighties.len());
    println!("nineties: {}", nineties.len());
}

fn jazz_releases<'a>(years: &'a [i64]) -> Releases<'a> {
    let eighties: &'a [i64] = &years[0..2];
    let nineties: &'a [i64] = &years[2..4];

    Releases {
        years: years,
        eighties: eighties,
        nineties: nineties,
    }
}
