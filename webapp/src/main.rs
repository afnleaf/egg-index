fn main() {
    let dozen_eggs_price = 4.146;
    let dozens_per_year = 52.0;
    let median_income = (51_350.0 + 35_410.0) / 2.0;
    let egg = (dozen_eggs_price * dozens_per_year) / median_income;

    let luxury_good = 150_000.0;
    let ceo_compensation = 14_724_000.0;
    let ceo = luxury_good / ceo_compensation;

    let egg_index = ceo / egg;

    let equivalent = ceo_compensation * egg;

    println!("egg:\t\t{}%\nceo:\t\t{}%\negg_index:\t{:.3}\neq:\t\t${:.2}", 
        egg*100.0, 
        ceo*100.0, 
        egg_index,
        equivalent
    );
}
