/*
Outline:
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
use std::collections::HashMap;

// Compute statistics (total, count, avg, median) of all transactions
pub fn compute_region_stats(transactions: &[Transaction]) -> Vec<RegionStats> {
    let mut map: HashMap<String, Vec<u64>> = HashMap::new(); // Hashmap <Region, [total, avg, med, count]

    for tx in transactions {
        // Group by location (as region)
        map.entry(tx.location.clone())
            .or_default()
            .push(tx.value);
    }

    let mut stats = Vec::new(); // Store RegionStats' components

    for (location, values) in map {
        let total: u64 = values.iter().sum();
        let count = values.len();
        let average = total as f64 / count as f64;
        let median = calculate_median(&values);

        stats.push(RegionStats {region: location, total, average, median, count});
    }

    stats.sort_by(|a, b| b.total.cmp(&a.total));
    stats
}

// Calculate median
fn calculate_median(value: &[f64]) -> f64 {
    // Sort the values
    let mut sorted = values.to_vec();
    sorted.sort_unstable();

    let n = sorted.len();
    let mid = n / 2;

    if n % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) as f64 / 2.0;
    } else {
        sorted[mid] as f64;
    }
}

// Aggregate transaction values & counts by date
pub fn aggregate_by_date(transactions: &[Transaction]) -> Vec<DayStats> {
    let mut agg_map: HashMap<NaiveData, (u64, u32)> == Hashmap::new(); // date, (value, transaction_count)

    for tx in transactions {
        let entry = agg_map.entry(tx.date).or_insert((0, 0)); // If date exists in hashmap, return a mut ref to the tuple (value, transaction_count)
        entry.0 += tx.value; 
        entry.1 += tx.transaction_count;
    }

    let mut result = Vec::new(); // Reformatting for output
    for (date, (value, transaction_count)) in agg_map {
        result.push(DayStats {
            date,
            value,
            transaction_count,
        });
    }

    result
}

// Detect anomaly - return the vector rows of the anomaly
pub fn detect_anomaly(transactions: &[Transaction]) -> Vec<RegionStats> {
    
}