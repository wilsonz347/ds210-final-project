/*
Outline:
- Handles reading and parsing the CSV dataset into usable Rust structs.
- CSV reader setup using csv crate
- Deserialize rows into Transaction structs
- Basic validation (e.g., invalid rows, empty fields)
- Function(s) returning Vec<Transaction>
*/

/* 
Testing:
- Test loading a small inline CSV using Cursor
- Validate that it handles missing/invalid data gracefully
- Check that Vec<Transaction> is parsed correctly
*/