use crate::models::{Transaction, RegionStats, DomainStats, DayStats};
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

    stats.sort_by(|a, b| a.region.cmp(&b.region));
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

// Aggregate transaction values & counts by date/domain
pub fn aggregate_by_date(transactions: &[Transaction]) -> Vec<DayStats> {
    let mut agg_map: HashMap<NaiveDate, (Vec<u64>, u32)> = HashMap::new(); // date, (values, transaction_count)

    for tx in transactions {
        let entry = agg_map.entry(tx.date).or_insert((Vec::new(), 0));
        entry.0.push(tx.value); // Store individual values
        entry.1 += tx.transaction_count;
    }

    let mut result = Vec::new();
    for (date, (values, transaction_count)) in agg_map {
        let total: u64 = values.iter().sum();
        let count = values.len();
        let average = if count > 0 { total as f64 / count as f64 } else { 0.0 };
        let median = calculate_median(&values);

        result.push(DayStats {
            date,
            value: total, // Sum of values
            transaction_count,
            average,
            median,
            count,
        });
    }

    result.sort_by(|a, b| a.date.cmp(&b.date));
    result
}

pub fn aggregate_by_domain(transactions: &[Transaction]) -> Vec<DomainStats> {
    let mut agg_map: HashMap<String, (Vec<u64>, u32)> = HashMap::new(); // domain, (values, transaction_count)

    for tx in transactions {
        let entry = agg_map.entry(tx.domain.clone()).or_insert((Vec::new(), 0));
        entry.0.push(tx.value); // Store individual values
        entry.1 += tx.transaction_count;
    }

    let mut new_vec = Vec::new();
    for (domain, (values, transaction_count)) in agg_map {
        let total: u64 = values.iter().sum();
        let count = values.len();
        let average = if count > 0 { total as f64 / count as f64 } else { 0.0 };
        let median = calculate_median(&values);

        new_vec.push(DomainStats {
            domain,
            value: total, // Sum of values
            transaction_count,
            average,
            median,
            count,
        });
    }

    new_vec.sort_by(|a, b| a.domain.cmp(&b.domain));
    new_vec
}

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

/* Test each functions */
/* SUGGESTIONS
- Create data visualizations (bar charts for region/domain || line graph for date)
*/