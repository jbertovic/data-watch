use crate::DataTimeStamp;
use std::time::Duration;
use async_trait::async_trait;
use xactor::*;
use super::{DataResponse, RequestSchedule, Refresh};
use surf::Request;
use log::{debug, info};

/// Creates a basic request that runs on a schedule and broadcasts `data_ts`

/// ReqBasic
/// Start - nothing on start
/// 
/// <RequestSchedule>
/// - store actor address and RequestSchedule message
/// - run request
/// - broadcast output message
/// 
/// <Refresh>
/// - run request
/// - broadcast output message
///
/// <Ping>

// IDEA: if things get slow, can we share a client amongst the actors or send the request to a separate broker who handles requests
// IDEA: or store the client state

#[derive(Default)]
pub struct ReqBasic {
    source_name: String,
    request: Option<Request>,
    translation: Option< fn(String) -> Vec<DataTimeStamp> >,
}

#[async_trait]
impl Actor for ReqBasic {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        // optional: do stuff on handler startup, like subscribing to a Broker
        // ctx.subscribe::<RequestSchedule>().await?;
        debug!("Actor::ReqBasic started");
        Ok(())
    }
}

#[async_trait]
impl Handler<RequestSchedule> for ReqBasic {
    async fn handle(&mut self, ctx: &mut Context<Self>, msg: RequestSchedule) {
        debug!("<RequestSchedule> received: {:?}", msg);
        info!("<RequestSchedule> received: {}", msg.source_name);
        self.source_name = msg.source_name;
        self.request = Some(surf::get(msg.api_url).build());
        self.translation = Some(msg.translation.clone());
        self.run_request().await;
        ctx.send_interval(Refresh{}, Duration::from_secs(msg.interval_sec));
    }
}

#[async_trait]
impl Handler<Refresh> for ReqBasic {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: Refresh) {
        info!("<Refresh> received:");
        self.run_request().await;
    }
}

impl ReqBasic {
    async fn run_request(&self) {
        let response = surf::client().recv_string(self.request.clone().unwrap()).await.unwrap();
        debug!("Response received: {:?}", response);
        let mut broker = Broker::from_registry().await.unwrap();
        let data = (self.translation.unwrap())(response);
        for data_point in data {
            info!("<DataResponse> publised");
            broker.publish(
                DataResponse{
                    source_name: self.source_name.clone(),
                    data_ts: data_point,
                }
            ).unwrap();
        }
    }
}