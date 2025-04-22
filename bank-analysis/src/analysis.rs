/*
Outline:
- Performs all analytics on the loaded transaction data.
- Compute total/average/median by region
- Aggregate transactions by day/month/year
- Detect anomalies using statistical thresholds (e.g., 99th percentile)
- Breakdown by transaction type or user behavior
- Return data structures suitable for CLI display or export
*/

/* 
Testing:
- Average, median, total calculation
- Region grouping and sorting logic
- Outlier detection (e.g., > 99th percentile)
- Aggregation by date
- Breakdown by transaction type
*/

use crate::models::{Transaction, RegionStats};

pub fn compute_region_stats(transactions: &[Transaction]) -> Vec<RegionStats> {
    // Print first 10 rows for inspection
    for (i, tx) in transactions.iter().take(10).enumerate() {
        println!("Row {}: {:?}", i + 1, tx);
    }

    Vec::new()
}