//fn main() {
//    let dozen_eggs_price = 4.146;
//    let dozens_per_year = 52.0;
//    let median_income = (51_350.0 + 35_410.0) / 2.0;
//    let egg = (dozen_eggs_price * dozens_per_year) / median_income;
//
//    let luxury_good = 150_000.0;
//    let ceo_compensation = 14_724_000.0;
//    let ceo = luxury_good / ceo_compensation;
//
//    let egg_index = ceo / egg;
//
//    let equivalent = ceo_compensation * egg;
//
//    println!("egg:\t\t{}%\nceo:\t\t{}%\negg_index:\t{:.3}\neq:\t\t${:.2}", 
//        egg*100.0, 
//        ceo*100.0, 
//        egg_index,
//        equivalent
//    );
//}

use std::path::Path;
use std::error::Error;
// crates
use serde::Deserialize;
use charts::{Chart, ScaleLinear, MarkerType, PointLabelPosition, LineSeriesView};

#[derive(Debug, Deserialize)]
struct Record {
    year: u16,
    egg_price: f32,
    egg_price_year: f32,
    median_income_male: f32,
    median_income_female: f32,
    median_income: f32,
    ceo_compensation: f32,
    equivalence: f32,
}

fn main() -> Result<(), Box<dyn Error>> {

    let path = Path::new("../data/cleansheet.csv");
    println!("{}", path.exists());

    // Create a CSV reader
    //let mut rdr = csv::Reader::from_path("../data/cleansheet.csv")?;
    let mut rdr = csv::Reader::from_path(path)?;

    // Read records
    let mut i = 0;
    for result in rdr.deserialize() {
        println!("test{}", i);
        if i == 0 {
            i += 1;
            continue;
        }
        let record: Record = result?;
        println!("{:?}", record);
        i += 1;
    }

    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

   // let x = ScaleLinear::new()
   //     .set_domain(vec![1980_f32, 2025_u16])
   //     .set_range(vec![0, width - left - right]);

   // let y = ScaleLinear::new()
   //     .set_domain(vec![0_f32, 75000000_f32])
   //     .set_range(vec![height - top - bottom, 0]);

    let x = ScaleLinear::new()
        .set_domain(vec![0_f32, 200_f32])
        .set_range(vec![0, width - left - right]);

   let y = ScaleLinear::new()
        .set_domain(vec![0_f32, 100_f32])
        .set_range(vec![height - top - bottom, 0]);

    let line_data = vec![(12, 54), (100, 40), (120, 50), (180, 70)];

    // Create Line series view that is going to represent the data.
    let line_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Circle)
        .set_label_position(PointLabelPosition::N)
        .load_data(&line_data).unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Line Chart"))
        .add_view(&line_view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Custom Y Axis Label")
        .add_bottom_axis_label("Custom X Axis Label")
        .save("line-chart.svg").unwrap();

    Ok(())
}
