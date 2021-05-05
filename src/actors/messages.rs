use xactor::*;
use serde::Deserialize;

#[message]
#[derive(Debug, Clone, Deserialize)]
pub struct RequestSchedule
{
    pub source_name: String,
    pub api_url: String,
    pub interval_sec: u64,
    pub jmespatch_query: String
}

#[message]
#[derive(Debug, Clone)]
pub struct Refresh;

#[message]
#[derive(Debug, Clone)]
pub struct DataResponse{
    pub source_name: String,
    pub measure_name: String,
    pub measure_desc: String,
    pub measure_value: f64,
    pub timestamp: u64 
}

#[message]
pub struct Stop;
