use super::producer::{ApiRequestType, ProducerAction};
use crate::SharedVar;
use xactor::*;

#[message]
#[derive(Debug, Clone)]
pub struct WebProducerSchedule {
    pub source_name: String,
    pub api_url: String,
    pub request_type: ApiRequestType,
    pub body: Option<String>,
    pub header: Option<(String, String)>,
    pub cron: String,
    pub jmespatch_query: String,
    pub storage_var: SharedVar,
    pub response_action: ProducerAction,
}

#[message]
#[derive(Debug, Clone)]
pub struct Refresh;

#[message]
#[derive(Debug, Clone)]
pub struct Run;

#[message]
#[derive(Debug, Clone)]
pub struct DataResponse {
    pub source_name: String,
    pub measure_name: String,
    pub measure_desc: String,
    pub measure_value: f64,
    pub timestamp: u64,
}

#[message]
pub struct Stop;
