use std::collections::HashMap;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use std::time::Duration;
use async_trait::async_trait;
use xactor::*;
use super::{DataResponse, RequestSchedule, Refresh};
use jmespatch::Expression;
// use crate::interpreter::interpret2;
use surf::Request;
use log::{debug, info};

/// Creates a basic request that runs on a schedule and broadcasts `data_ts`

/// RequestJson
/// Start - nothing on start
/// 
/// <RequestSchedule>
/// - store state for source_name, http client, translation of data
/// - run request and broadcast output message
/// - set interval to send <Refresh>
/// 
/// <Refresh>
/// - run request
/// - broadcast output message
///
/// <Ping>

// IDEA: if things get slow, can we share a client amongst the actors or send the request to a separate broker who handles requests
// IDEA: or store the client state

#[derive(Default)]
pub struct RequestJson {
    source_name: String,
    request: Option<Request>,
    translation: Option<Expression<'static>>,
}

#[async_trait]
impl Actor for RequestJson {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        // optional: do stuff on handler startup, like subscribing to a Broker
        // ctx.subscribe::<RequestSchedule>().await?;
        debug!("Actor::ReqBasic started");
        Ok(())
    }
}

#[async_trait]
impl Handler<RequestSchedule> for RequestJson {
    async fn handle(&mut self, ctx: &mut Context<Self>, msg: RequestSchedule) {
        debug!("<RequestSchedule> received: {:?}", msg);
        info!("<RequestSchedule> received: {}", msg.source_name);
        self.source_name = msg.source_name;
        self.request = Some(surf::get(msg.api_url).build());
        self.translation = Some(jmespatch::compile(msg.jmespatch_query.as_ref()).unwrap());
        self.run_request().await;
        ctx.send_interval(Refresh{}, Duration::from_secs(msg.interval_sec));
    }
}

#[async_trait]
impl Handler<Refresh> for RequestJson {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: Refresh) {
        info!("<Refresh> received:");
        self.run_request().await;
    }
}

impl RequestJson {
    /// use http client to make request
    async fn run_request(&self) {
        let response = surf::client().recv_string(self.request.clone().unwrap()).await.unwrap();
        debug!("Response received: {:?}", response);
        let data_received = self.parse_json(&response);
        self.publish_data(data_received).await;
    }

    /// parse raw json into Hashmap of data
    /// 
    /// jmespath parse returns: 
    /// 
    /// Multiple measures in one query (includes multiple measure types)
    /// [
    ///     { measure_name: "", measure_data: {measure_desc1: #, measure_desc2: #} },
    ///     { measure_name: "", measure_data: {measure_desc1: #, measure_desc2: #} },
    /// ]
    /// 
    /// OR Single measure in one query (includes multiple measure types)
    /// { measure_name: "", measure_data: {measure_desc1: #, measure_desc2: #} }
    /// 
    /// 
    /// Return should be Hashmap<&str, Vec<(&str, f64)>
    /// <measure_name, Vec<measure_desc, measure_value)>>
    fn parse_json(&self, json_response: &str) -> HashMap<String, Vec<(String, f64)>> {
        let parsed_json = jmespatch::Variable::from_json(json_response).unwrap();
        let result = self.translation.as_ref().unwrap().search(parsed_json).unwrap();
        let mut out = HashMap::new();
        debug!("Parsed result: {:?}", result);
        // decide if array or object (ie multiple measures or one measure with multiple data descriptions)
        if result.is_object() {
            let measure_name = result.as_object().unwrap().get("measure_name").unwrap().as_string().unwrap().to_owned();
            let mut data_points = Vec::new();
            for entry in result.as_object().unwrap().get("measure_data").unwrap().as_object().unwrap() {
                data_points.push((
                    entry.0.to_owned(),
                    entry.1.as_number().unwrap(),
                ))
            }
            out.insert(measure_name, data_points);
        }
        else if result.is_array() {
            unimplemented!();
        }
        out
    }

    async fn publish_data(&self, data_response: HashMap<String, Vec<(String, f64)>>) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut broker = Broker::from_registry().await.unwrap();
        for entry in data_response{
            let measure_name = entry.0;
            for data in entry.1 {
                broker.publish(
                    DataResponse{
                        source_name: self.source_name.to_owned(),
                        measure_name: measure_name.to_owned(),
                        measure_desc: data.0,
                        measure_value: data.1,
                        timestamp,
                    }
                ).unwrap();
            }
        }
    }

}