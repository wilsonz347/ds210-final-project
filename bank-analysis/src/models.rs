use chrono::NaiveDate;
use serde::Deserialize;

// For analysis: convert transactions struct to RegionStats
// Necessary for graphing regular scatter plots later on
#[derive(Debug)]
pub struct RegionStats {
    pub region: String,
    pub total: u64,
    pub average: f64,
    pub median: f64,
    pub count: usize,
}

// For analysis: convert transactions struct to MonthStats
// Necessary for graphing time series later on
#[derive(Debug)]
pub struct MonthStats {
    pub month: u32,        
    pub value: u64,
    pub transaction_count: u32,
    pub average: f64,
    pub median: f64,
    pub count: usize,
}

// Main struct, used as inputs for analysis functions
#[derive(Debug, Deserialize, Clone)]
pub struct Transaction {
    #[serde(deserialize_with = "parse_date")]
    pub date: NaiveDate,
    pub domain: String,
    pub location: String,
    pub value: u64,
    pub transaction_count: u32,
}

// AI: Custom parsing for NaiveDate
fn parse_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%m/%d/%Y").map_err(serde::de::Error::custom)
}