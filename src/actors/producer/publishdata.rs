use crate::actors::messages::DataResponse;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use xactor::{Broker, Service};

/// publish data in DataResponse format
pub async fn publish_data(source_name: &str, data_response: HashMap<String, Vec<(String, f64)>>) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut broker = Broker::from_registry().await.unwrap();
    for entry in data_response {
        let measure_name = entry.0;
        for data in entry.1 {
            broker
                .publish(DataResponse {
                    source_name: source_name.to_owned(),
                    measure_name: measure_name.to_owned(),
                    measure_desc: data.0,
                    measure_value: data.1,
                    timestamp,
                })
                .unwrap();
        }
    }
}
