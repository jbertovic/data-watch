use async_trait::async_trait;
use xactor::*;
use super::messages::{DataResponse};

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
pub struct StdoutWriter;

#[async_trait]
impl Actor for StdoutWriter {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        ctx.subscribe::<DataResponse>().await?;
        // println!("Actor::DataWriter started");
        Ok(())
    }
}

#[async_trait]
impl Handler<DataResponse> for StdoutWriter {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: DataResponse) {
        // println!("Actor::DataWriter message<DataResponse> received");
        println!("Name: {}, {:?}", msg.source_name, msg.data_ts);
    }
}

