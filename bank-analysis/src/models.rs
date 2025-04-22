/* 
Outline:
- Any additional types (e.g., TrendSummary, OutlierInfo) used in analysis
*/

use chrono::NaiveDate;
use serde::Deserialize;

pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
}

#[derive(Debug)]
pub struct RegionStats {
    pub region: String,
    pub total: u64,
    pub average: f64,
    pub median: f64,
    pub count: usize,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(deserialize_with = "parse_date")]
    pub date: NaiveDate,
    pub domain: String,
    pub location: String,
    pub value: u64,
    pub transaction_count: u32,
}

fn parse_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%m/%d/%Y").map_err(serde::de::Error::custom)
}