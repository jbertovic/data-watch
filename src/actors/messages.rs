use xactor::*;
use crate::DataTimeStamp;
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
    pub data_ts: DataTimeStamp,
}

#[message]
pub struct Stop;
