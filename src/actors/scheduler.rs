use super::messages::{Refresh, Stop, WebProducerSchedule};
use super::producer::WebProducer;
use async_trait::async_trait;
use log::{debug, info};
use xactor::*;

/// Scheduler
/// Start - subscribed to <RequestSchedule>
///
/// <RequestSchedule>
/// - spawn actor to handle new schedule
/// - send message to actor
/// - store actor address and RequestSchedule message
///
/// <StopSchedule>
///
/// <ListSchedule>
///
///
/// <Ping>

#[derive(Default)]
pub struct Scheduler {
    scheduled: Vec<WebProducerSchedule>,
    actors: Vec<Addr<WebProducer>>,
}

#[async_trait]
impl Actor for Scheduler {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("Actor::Scheduler started");
        Ok(())
    }

    async fn stopped(&mut self, _: &mut Context<Self>) {
        info!("Scheduler Stopped");
    }
}

#[async_trait]
impl Handler<WebProducerSchedule> for Scheduler {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: WebProducerSchedule) {
        debug!("message<RequestSchedule> received: {:?}", msg);
        info!("<RequestSchedule> received: {}", msg.source_name);
        // create new actor to manage request
        self.scheduled.push(msg.clone());

        let newactor = WebProducer::new(msg.clone()).start().await.unwrap();

        self.actors.push(newactor.clone());

        newactor.send(Refresh {}).unwrap();
    }
}

#[async_trait]
impl Handler<Stop> for Scheduler {
    async fn handle(&mut self, ctx: &mut Context<Self>, _msg: Stop) {
        info!("<Stop> received");
        ctx.stop(None);
    }
}
