use crate::SharedVar;
use xactor::*;
use super::ResponseAction;

#[message]
#[derive(Debug, Clone)]
pub struct RequestSchedule
{
    pub source_name: String,
    pub api_url: String,
    pub interval_sec: u64,
    pub jmespatch_query: String,
    pub storage_var: SharedVar,
    pub response_action: ResponseAction,
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
