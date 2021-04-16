use xactor::*;
use crate::DataTimeStamp;

#[message]
#[derive(Debug, Clone)]
pub struct RequestSchedule {
    pub source_name: String,
    pub api_url: String,
    pub interval_sec: u64,
    pub translation: fn (String) -> Vec<DataTimeStamp>,
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
