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
use charts::{Chart, ScaleLinear, MarkerType, PointLabelPosition, LineSeriesView, Color};
use maud::{DOCTYPE, html, Markup};

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

fn create_graph_chart() -> Result<(), Box<dyn Error>> {

    let path = Path::new("../data/cleansheet.csv");
    println!("{}", path.exists());

    // Create a CSV reader
    //let mut rdr = csv::Reader::from_path("../data/cleansheet.csv")?;
    let mut rdr = csv::Reader::from_path(path)?;

    // Read records
    let mut i = 0;
    let mut records = vec![];
    for result in rdr.deserialize() {
        println!("test{}", i);
        if i == 0 {
            i += 1;
            continue;
        }
        let record: Record = result?;
        //println!("{:?}", record);
        records.push(record);
        i += 1;
    }

    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    let x = ScaleLinear::new()
       .set_domain(vec![1980_f32, 2025_f32])
       .set_range(vec![0, width - left - right]);

    let y = ScaleLinear::new()
       .set_domain(vec![0_f32, 100000_f32])
       .set_range(vec![height - top - bottom, 0]);

    //let x = ScaleLinear::new()
    //     .set_domain(vec![0_f32, 200_f32])
    //     .set_range(vec![0, width - left - right]);

    //let y = ScaleLinear::new()
    //     .set_domain(vec![0_f32, 100_f32])
    //     .set_range(vec![height - top - bottom, 0]);

    //let line_data = vec![(12, 54), (100, 40), (120, 50), (180, 70)];

    let mut egg_price_year_data = vec![];
    let mut median_income_data = vec![];
    let mut ceo_comp_data = vec![];
    let mut equivalence_data = vec![];
    //let line_data = vec![];
    for r in records.iter() {
        egg_price_year_data.push((r.year as f32, r.egg_price_year));
        median_income_data.push((r.year as f32, r.median_income));
        ceo_comp_data.push((r.year as f32, r.ceo_compensation));
        equivalence_data.push((r.year as f32, r.equivalence));
        println!("{:?}", r);
    } 

    let egg_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Circle)
        .set_label_visibility(false)
        .set_colors(Color::from_vec_of_hex_strings(vec!["#f5f4df"]))
        .load_data(&egg_price_year_data).unwrap();

    let med_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Circle)
        .set_label_visibility(false)
        .set_colors(Color::from_vec_of_hex_strings(vec!["#183166"]))
        .load_data(&median_income_data).unwrap();

    let ceo_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Circle)
        .set_label_visibility(false)
        .set_colors(Color::from_vec_of_hex_strings(vec!["#488a2c"]))
        .load_data(&ceo_comp_data).unwrap();

    let eq_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Circle)
        .set_label_visibility(false)
        .set_colors(Color::from_vec_of_hex_strings(vec!["#aa0000"]))
        .load_data(&equivalence_data).unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Median Income vs CEO Income (Annual)"))
        .add_view(&egg_view)
        .add_view(&med_view)
        .add_view(&ceo_view)
        .add_view(&eq_view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("USD $$$")
        .add_bottom_axis_label("Year")
        .save("line-chart.svg").unwrap();

    Ok(())
}

/// A basic header with a dynamic `page_title`.
fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
    }
}

/// A static footer.
fn footer() -> Markup {
    html! {
        footer {
            a href="rss.atom" { "RSS Feed" }
        }
    }
}

/// The final Markup, including `header` and `footer`.
///
/// Additionally takes a `greeting_box` that's `Markup`, not `&str`.
pub fn page(title: &str) -> Markup {
    html! {
        // Add the header markup to the page
        (header(title))
        h1 { (title) }
        (footer())
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    create_graph_chart();
   
    let markup = page("eggs");

    println!("{}", markup.into_string());

    Ok(())
}
