use async_trait::async_trait;
use xactor::*;
use crate::actors::messages::{DataResponse};

/// Outputs to writer which is currently stdout, but could switch in the future
/// could include state to use with writer

/// DataWriter
/// Start - subscribed to <DataResponse>
/// 
/// <DataResponse>
/// - print DataTimeseries to screen
/// 
/// <Ping>


#[derive(Default)]
pub struct StdoutConsumer;

#[async_trait]
impl Actor for StdoutConsumer {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        ctx.subscribe::<DataResponse>().await?;
        // println!("Actor::DataWriter started");
        Ok(())
    }
}

#[async_trait]
impl Handler<DataResponse> for StdoutConsumer {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: DataResponse) {
        // println!("Actor::DataWriter message<DataResponse> received");
        println!("{:?}", msg);
    }
}

