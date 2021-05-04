use async_trait::async_trait;
use xactor::*;
use super::RequestSchedule;
use super::Stop;
use super::RequestJson;
use log::{info, debug};

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


pub struct Scheduler {
    scheduled: Vec<RequestSchedule>,
    actors: Vec<Addr<RequestJson>>,
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
impl Handler<RequestSchedule> for Scheduler {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: RequestSchedule) {
        debug!("message<RequestSchedule> received: {:?}", msg);
        info!("<RequestSchedule> received: {}", msg.source_name);
        // create new actor to manage request
        self.scheduled.push(msg.clone());
        let newactor = RequestJson::default().start().await.unwrap();
        self.actors.push(newactor.clone());
        newactor.send(msg).unwrap();
    }
}

#[async_trait]
impl Handler<Stop> for Scheduler {
    async fn handle(&mut self, ctx: &mut Context<Self>, _msg: Stop) {
        info!("<Stop> received");
        ctx.stop(None);
    }
}


impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            scheduled: Vec::new(),
            actors: Vec::new(),
        }
    }
}