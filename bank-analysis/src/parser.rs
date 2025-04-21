use serde::Deserialize; // Helps to convert columns into struct
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use chrono::NaiveDate; // Helps with converting time (string) to the correct format

#[derive(Debug, Deserialize)]
struct Transaction {
    #[serde(deserialize_with = "parse_date")]
    date: NaiveDate,
    domain: String,
    location: String,
    value: u64,
    transaction_count: u32,
}

// Function to parse the date (CUSTOM)
fn parse_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%m/%d/%Y").map_err(serde::de::Error::custom)
}

// Function to load and read the csv file
fn load_csv_file(url: &str) -> Result<Vec<Transaction>, Box<dyn Error>>{
    let file = File::open(url)?; // Will return Err if file can’t open
    let mut reader = ReaderBuilder::new()
        .has_headers(true) // Skip first row
        .from_reader(file);

    let mut column_items = Vec::new();

    for result in reader.deserialize() {
        let mut transaction: Transaction = result?; // Return error if failure

        // Clean up any extra whitespace in the fields
        transaction.domain = transaction.domain.trim().to_string();
        transaction.location = transaction.location.trim().to_string();

        if transaction.domain == "RESTRAUNT" {
            transaction.domain = "RESTAURANT".to_string(); // Replace with the correct term
        }
        column_items.push(transaction);
    }

    Ok(column_items)
}

#[test]
fn test_load_file() {
    use std::io::Cursor; // Pretend string as file for reading

    // Sample data
    let data = "\
date,domain,location,value,transaction_count
1/1/2022,RESTRAUNT,Goa,1000,2
";

    // Initialize cursor & reader
    let cursor = Cursor::new(data);

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(cursor);

    let mut results = Vec::new();

    for result in reader.deserialize() {
        let mut transaction: Transaction = result.expect("Failed to parse row"); // Program crash if fail

        // Remove white spaces
        transaction.domain = transaction.domain.trim().to_string();
        transaction.location = transaction.location.trim().to_string();

        if transaction.domain == "RESTRAUNT" {
            transaction.domain = "RESTAURANT".to_string();
        }

        results.push(transaction);
    }

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].domain, "RESTAURANT");
    assert_eq!(results[0].location, "Goa");
    assert_eq!(results[0].value, 1000);
    assert_eq!(results[0].transaction_count, 2);
}   