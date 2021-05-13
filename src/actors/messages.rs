use crate::SharedVar;
use xactor::*;
use super::{ResponseAction, RequestType};

#[message]
#[derive(Debug, Clone)]
pub struct RequestSchedule
{
    pub source_name: String,
    pub api_url: String,
    pub request_type: RequestType,
    pub body: Option<String>,
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
