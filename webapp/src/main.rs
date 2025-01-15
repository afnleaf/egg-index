fn main() {
    let dozen_eggs_price = 3.50;
    let dozens_per_year = 52.0;
    let median_income = 41_343.0;
    let egg = (dozen_eggs_price * dozens_per_year) / median_income;

    let luxury_good = 150_000.0;
    let ceo_compensation = 55_000_000.0;
    let ceo = luxury_good / ceo_compensation;

    let egg_index = ceo / egg;

    println!("egg:\t\t{}%\nceo:\t\t{}%\negg_index:\t{:.3}", 
        egg*100.0, 
        ceo*100.0, 
        egg_index
    );
}
