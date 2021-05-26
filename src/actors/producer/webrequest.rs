use crate::actors::messages::{DataResponse, WebProducerSchedule, Refresh};
use crate::utility;
use super::{ProducerAction, WebRequestType};
use std::collections::HashMap;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use std::time::Duration;
use async_trait::async_trait;
use xactor::*;
use jmespatch::Expression;
use log::{debug, info};
use http_types::mime;

/// Creates a web API request that runs on a schedule and publishes data
/// uses jmespath expression to parse out relevant data

/// RequestJson
/// Start - nothing on start
/// 
/// <RequestSchedule>
/// - store state request description
/// - run request and broadcast output message or store variable
/// - set interval to send <Refresh> unless Interval = zero than one-time request (NOT IMPLEMENTED)
/// 
/// <Refresh>
/// - run request
/// - broadcast output message OR store variable based on RequestAction
///
/// <Ping> (NOT IMPLEMENTED)

// IDEA: if things get slow, can we share a client amongst the actors or send the request to a separate broker who handles requests
// IDEA: or store the client state.  currently the request is rebuilt each time



pub struct WebProducer {
    translation: Expression<'static>,
    request_description: WebProducerSchedule,
}

#[async_trait]
impl Actor for WebProducer {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        // optional: do stuff on handler startup, like subscribing to a Broker
        // ctx.subscribe::<RequestSchedule>().await?;
        debug!("Actor::ReqBasic started");
        Ok(())
    }
}

#[async_trait]
impl Handler<WebProducerSchedule> for WebProducer {
    async fn handle(&mut self, ctx: &mut Context<Self>, msg: WebProducerSchedule) {
        debug!("<RequestSchedule> received: {:?}", msg);
        info!("<RequestSchedule> received: {}", msg.source_name);
        self.run_request().await;
        ctx.send_interval(Refresh{}, Duration::from_secs(msg.interval_sec));
    }
}

#[async_trait]
impl Handler<Refresh> for WebProducer {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: Refresh) {
        info!("<Refresh> received:");
        self.run_request().await;
    }
}

impl WebProducer {

    pub fn new(request_description: WebProducerSchedule) -> Self {
        let translation = jmespatch::compile(request_description.jmespatch_query.as_ref()).unwrap();
        WebProducer {
            translation,
            request_description,
        }
    }

    /// Builds and runs request
    async fn run_request(&mut self) {
        // build body if it exists and attach
        // set mime type to form
        // swap variables in for [[ ]]
        let api_url = utility::swap_variable(&self.request_description.storage_var, &self.request_description.api_url, true);

        let mut request = match self.request_description.request_type {
            WebRequestType::GET => surf::get(api_url),
            WebRequestType::POST => {
                let body = match self.request_description.body.clone() {
                    Some(b) => utility::swap_variable(&self.request_description.storage_var, &b, true),
                    None => String::from(""),
                };
                surf::post(api_url).body(body).content_type(mime::FORM)
            }
        };

        // set header
        if let Some((key, value)) = &self.request_description.header {
            request = request.header(key.as_str(), utility::swap_variable(&self.request_description.storage_var, &value, false));
        };

        let response = request.recv_string().await.unwrap();

        debug!("Response received: {:?}", &response);

        match &self.request_description.response_action {
            ProducerAction::PUBLISHDATA => {
                let data_received = utility::parse_json(&self.translation, &response);
                self.publish_data(data_received).await;
            },
            ProducerAction::STOREVARIABLE => {
                utility::store_variable(&self.translation, &self.request_description.storage_var, &response);
            }
        }
    }

    /// publish data in DataResponse format
    async fn publish_data(&self, data_response: HashMap<String, Vec<(String, f64)>>) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut broker = Broker::from_registry().await.unwrap();
        for entry in data_response{
            let measure_name = entry.0;
            for data in entry.1 {
                broker.publish(
                    DataResponse{
                        source_name: self.request_description.source_name.to_owned(),
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
