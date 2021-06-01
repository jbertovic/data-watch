use crate::actors::messages::{Refresh, Run, WebProducerSchedule};
use crate::actors::producer::{apirequest::request_api, publishdata::publish_data, ProducerAction};
use crate::utility;
use async_trait::async_trait;
use chrono::Utc;
use cron::Schedule;
use jmespatch::Expression;
use log::{debug, info};
use std::str::FromStr;
use std::time::Duration;
use xactor::*;

/// Creates a web API request that runs on a schedule and publishes data
/// uses jmespath expression to parse out relevant data
/// uses cron expression to determining timing of stream

// IDEA: if things get slow, can we share a client amongst the actors or send the request to a separate broker who handles requests
// IDEA: or store the client state.  currently the request is rebuilt each time

pub struct WebProducer {
    translation: Expression<'static>,
    schedule: Schedule,
    request_description: WebProducerSchedule,
}

#[async_trait]
impl Actor for WebProducer {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        // optional: do stuff on handler startup, like subscribing to a Broker
        // ctx.subscribe::<RequestSchedule>().await?;
        debug!(
            "Actor::WebProducer started for {}",
            &self.request_description.source_name
        );
        Ok(())
    }
}

#[async_trait]
impl Handler<Refresh> for WebProducer {
    async fn handle(&mut self, ctx: &mut Context<Self>, _msg: Refresh) {
        info!(
            "<Refresh> received for {}:",
            &self.request_description.source_name
        );
        let next = self.schedule.upcoming(Utc).next().unwrap();
        let diff = next - Utc::now() + chrono::Duration::milliseconds(100);
        ctx.send_later(Run {}, diff.to_std().unwrap());
    }
}

#[async_trait]
impl Handler<Run> for WebProducer {
    async fn handle(&mut self, ctx: &mut Context<Self>, _msg: Run) {
        info!(
            "<Run> received for {}:",
            &self.request_description.source_name
        );
        self.run_request().await;
        ctx.send_later(Refresh {}, Duration::from_millis(100));
    }
}

impl WebProducer {
    pub fn new(request_description: WebProducerSchedule) -> Self {
        let translation = jmespatch::compile(request_description.jmespatch_query.as_ref()).unwrap();
        let schedule = Schedule::from_str(request_description.cron.as_str()).unwrap();
        WebProducer {
            translation,
            schedule,
            request_description,
        }
    }

    /// Builds and runs request
    async fn run_request(&mut self) {
        // swap variables in api_url, body, header for [[ ]]
        let api_url = utility::swap_variable(
            &self.request_description.storage_var,
            &self.request_description.api_url,
            true,
        );
        let body = match self.request_description.body.clone() {
            Some(b) => utility::swap_variable(&self.request_description.storage_var, &b, true),
            None => String::from(""),
        };
        let header = match &self.request_description.header {
            Some((key, value)) => {
                let new_value =
                    utility::swap_variable(&self.request_description.storage_var, &value, false);
                Some((key.as_str(), new_value))
            }
            None => None,
        };

        let response = request_api(
            &self.request_description.request_type,
            &api_url,
            &body,
            header,
        )
        .await;

        self.response_action(&response).await;
    }

    async fn response_action(&self, response: &String) {
        match &self.request_description.response_action {
            ProducerAction::PUBLISHDATA => {
                let data_received = utility::parse_json(&self.translation, &response);
                publish_data(&self.request_description.source_name, data_received).await;
            }
            ProducerAction::STOREVARIABLE => {
                utility::store_variable(
                    &self.translation,
                    &self.request_description.storage_var,
                    &response,
                );
            }
        }
    }
}
