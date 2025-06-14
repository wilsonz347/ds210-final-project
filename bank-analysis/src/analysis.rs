use crate::models::{Transaction, RegionStats, MonthStats};
use std::collections::HashMap;
use chrono::Datelike;

// Compute statistics (total, count, avg, median) of all transactions
// Loops over the transactions struct and push them into the hashmap for analysis
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

    stats.sort_by(|a, b| a.region.cmp(&b.region));
    stats
}

// Calculate median (for the aggregation functions)
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

// Aggregate transaction values & counts by month
// Loops over the transactions struct and push them into the hashmap for analysis
// Use mathematical computations to determine average, median, etc
pub fn aggregate_by_month(transactions: &[Transaction]) -> Vec<MonthStats> {
    let mut agg_map: HashMap<u32, (Vec<u64>, u32)> = HashMap::new(); // month -> (values, transaction_count)

    for tx in transactions {
        let month = tx.date.month();
        let entry = agg_map.entry(month).or_insert((Vec::new(), 0));
        entry.0.push(tx.value);
        entry.1 += tx.transaction_count;
    }

    let mut result = Vec::new();
    for (month, (values, transaction_count)) in agg_map {
        let total: u64 = values.iter().sum();
        let count = values.len();
        let average = if count > 0 { total as f64 / count as f64 } else { 0.0 };
        let median = calculate_median(&values);

        result.push(MonthStats {
            month,
            value: total,
            transaction_count,
            average,
            median,
            count,
        });
    }

    result.sort_by_key(|m| m.month);
    result
}

// Custom percentile function for the detect anomaly functions
fn percentile(values: Vec<u64>, p: f64) -> f64 {
    let mut sorted = values;
    sorted.sort_unstable(); // Sort in-place

    let position = (p * (sorted.len() - 1) as f64).round() as usize;
    sorted[position] as f64
}

// Detect anomaly - return the vector rows of the anomaly (for value & transaction count)
pub fn detect_anomaly_for_value(transactions: &[Transaction]) -> Vec<Transaction> {
    let values: Vec<u64> = transactions.iter().map(|tx| tx.value).collect(); // [365554, 584958, 885720, ...]

    // IQR (Q3 - Q1)
    let q3 = percentile(values.clone(), 0.75);
    let q1 = percentile(values, 0.25);
    let iqr = q3 - q1;

    // Find the lower & upper bound
    let lower = q1 - 1.5 * iqr;
    let upper = q3 + 1.5 * iqr;

    let outliers = transactions.iter().filter(|tx| (tx.value as f64) < lower || (tx.value as f64) > upper).cloned().collect(); // Cloned for mapping clone to each value

    outliers
}

// Detect anomaly for transaction count instead of value
pub fn detect_anomaly_for_transaction_count(transactions: &[Transaction]) -> Vec<Transaction> {
    let transaction_counts: Vec<u64> = transactions.iter().map(|tx| tx.transaction_count as u64).collect();

    let q3 = percentile(transaction_counts.clone(), 0.75); 
    let q1 = percentile(transaction_counts, 0.25);
    let iqr = q3 - q1;

    let lower = q1 - 1.5 * iqr;
    let upper = q3 + 1.5 * iqr;

    let outliers = transactions.iter().filter(|tx| (tx.transaction_count as f64) < lower || (tx.transaction_count as f64) > upper).cloned().collect();

    outliers
}

#[test]
// Test percentile & median function
fn test_percentile() {
    let values = vec![12, 7, 22, 15, 9, 30, 18, 5, 14, 10];
    let target_num = 9.0; // 25th percentile (rounded)
    let func_num = percentile(values, 0.25);

    assert_eq!(target_num, func_num);
}

#[test]
fn test_median() {
    let values = vec![12, 7, 22, 15, 9, 30, 18, 5, 14, 10];
    let target_median = 13.0; 
    let func_num = calculate_median(&values);

    assert_eq!(target_median, func_num);
}