enum CitySize {
    Town,       // approximate residents: 1_000
    City,       // approximate residents: 10_000
    Metropolis, // approximate residents: 1_000_000
}

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

impl City {
    fn new(city_size: CitySize, is_coastal: bool) -> City {
        let (description, residents) = match city_size {
            CitySize::Town => {
                let residents = 1_000;

                (
                    format!("a *town* of approximately {} residents", residents),
                    residents,
                )
            }
            CitySize::City => {
                let residents = 10_000;

                (
                    format!("a *city* of approximately {} residents", residents),
                    residents,
                )
            }
            CitySize::Metropolis => {
                let residents = 1_000_000;

                (
                    format!("a *metropolis* of approximately {} residents", residents),
                    residents,
                )
            }
        };

        City {
            description,
            residents,
            is_coastal,
        }
    }
}

fn main() {
    let metroville = City::new(CitySize::Metropolis, false);
    let cityville = City::new(CitySize::City, true);
    let townville = City::new(CitySize::Town, true);


    println!("This metro is {}", metroville.description);
    println!("This city is {}", cityville.description);
    println!("This town is {}", townville.description);

    println!("This metro has beach {}", metroville.is_coastal);
    println!("This city has beach {}", cityville.is_coastal);
    println!("This town has beach {}", townville.is_coastal);


    if metroville.residents > 100_000 {
        println!("Wow!");
    }
}
