/* 
Outline: 
- Parse CLI arguments (optional: use clap or std::env)
- Call functions from parser.rs and analysis.rs
- Display top results in the terminal (e.g., top regions)
- Optionally export results as CSV/JSON directly from here
- Handle error output and graceful failure
*/

mod parser;
mod analysis;
mod models;

use crate::parser::load_csv_file;
use crate::analysis::{compute_region_stats, aggregate_by_date, aggregate_by_domain};

fn main() {
    let transactions = load_csv_file("../data/bankdataset.csv").expect("Failed to load");

    let region_stats = compute_region_stats(&transactions);

    let date_stats = aggregate_by_date(&transactions);

    let domain_stats = aggregate_by_domain(&transactions);

    for stat in region_stats {
        println!("{:?}", stat);
    }

    for stat in date_stats {
        println!("{:?}", stat);
    }

    for stat in domain_stats {
        println!("{:?}", stat);
    }
}
