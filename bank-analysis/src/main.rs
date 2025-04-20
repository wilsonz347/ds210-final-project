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

fn main() {
    println!("Hello World");
}