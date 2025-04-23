/*
Outline:
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

use crate::models::{Transaction, RegionStats, DayStats};
use std::collections::HashMap;
use chrono::NaiveDate;

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
fn calculate_median(values: &[u64]) -> f64 {
    // Sort the values
    let mut sorted = values.to_vec();
    sorted.sort_unstable(); // for u64 type

    let n = sorted.len();
    let mid = n / 2;

    if n % 2 == 0 {
        return (sorted[mid - 1] + sorted[mid]) as f64 / 2.0;
    } else {
        return sorted[mid] as f64;
    }
}

// Aggregate transaction values & counts by date
pub fn aggregate_by_date(transactions: &[Transaction]) -> Vec<DayStats> {
    let mut agg_map: HashMap<NaiveDate, (u64, u32)> = HashMap::new(); // date, (value, transaction_count)

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

fn percentile(mut values: Vec<u64>, p: f64) -> f64 {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let position = (p * (values.len() - 1) as f64).round() as usize; // Convert to usize for indexing, no interpolation

    values[position] as f64
}

// Detect anomaly - return the vector rows of the anomaly (for value & transaction count)
pub fn detect_anomaly_for_value(transactions: &[Transaction]) -> Vec<Transaction> {
    let mut values: Vec<u64> = transactions.iter().map(|tx| tx.value).collect(); // [365554, 584958, 885720, ...]
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // IQR (Q3 - Q1)
    let q1 = percentile(values.clone(), 25.0); 
    let q3 = percentile(values.clone(), 75.0);
    let iqr = q3 - q1;

    // Find the lower & upper bound
    let lower = q1 - 1.5 * iqr;
    let upper = q3 + 1.5 * iqr;

    let outliers = transactions.iter().filter(|tx| (tx.value as f64) < lower || (tx.value as f64) > upper).cloned().collect(); // Cloned for mapping clone to each value

    outliers
}

pub fn detect_anomaly_for_transaction_count(transactions: &[Transaction]) -> Vec<Transaction> {
    let mut transaction_counts: Vec<u64> = transactions.iter().map(|tx| tx.transaction_count as u64).collect();
    transaction_counts.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let q1 = percentile(transaction_counts.clone(), 25.0); 
    let q3 = percentile(transaction_counts.clone(), 75.0);
    let iqr = q3 - q1;

    let lower = q1 - 1.5 * iqr;
    let upper = q3 + 1.5 * iqr;

    let outliers = transactions.iter().filter(|tx| (tx.transaction_count as f64) < lower || (tx.transaction_count as f64) > upper).cloned().collect();

    outliers
}

pub fn aggregate_by_domain(transactions: &[Transaction]) -> Vec<Transaction> {
    Vec::new() // placeholder
}