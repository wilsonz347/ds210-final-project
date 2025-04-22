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
use crate::analysis::compute_region_stats;

fn main() {
    let transactions = load_csv_file("../data/bankdataset.csv").expect("Failed to load");

    let region_stats = compute_region_stats(&transactions);

    for stat in region_stats {
        println!("{:?}", stat);
    }
}
