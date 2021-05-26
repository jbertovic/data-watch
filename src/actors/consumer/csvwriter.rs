//  TODO: Have a file name created for name of measure set on initiation of actor
//   - don't try to do this with every measure to store or you can combine measures to store
//   - maybe keep a vector of names to store in file
//   - or create a unique routing name from name+description?

use async_trait::async_trait;
use async_std::{fs::File, io::{BufWriter, prelude::WriteExt}};
use xactor::*;
use crate::actors::messages::DataResponse;
use log::{info};

/// Outputs to writer which is currently stdout, but could switch in the future
/// could include state to use with writer

/// DataWriter
/// Start - subscribed to <DataResponse>
/// 
/// <DataResponse>
/// - print DataTimeseries to screen
/// 
/// <Ping>


pub struct CsvConsumer {
    writer: BufWriter<File>,
}

impl CsvConsumer {
    pub fn new(writer: BufWriter<File>) -> CsvConsumer {
        CsvConsumer {
            writer: writer,
        }
    }
}

impl Default for CsvConsumer {
   fn default() ->  CsvConsumer {
        async_std::task::block_on(
            async {
                let file = File::create("data.csv").await.unwrap();
                CsvConsumer {
                    writer: BufWriter::new(file),
                } 
            }
        )
    }
}

#[async_trait]
impl Actor for CsvConsumer {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        info!("CsvWriter started");
        ctx.subscribe::<DataResponse>().await?;
        Ok(())
    }
}

#[async_trait]
impl Handler<DataResponse> for CsvConsumer {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: DataResponse) {
        // append to csv file stream
        self.writer.write(format!("\"{}\", \"{}\", \"{}\", {}, {}\n", 
            msg.source_name, 
            msg.measure_name,
            msg.measure_desc,
            msg.measure_value,
            msg.timestamp,
        ).as_bytes()).await.unwrap();
        self.writer.flush().await.unwrap();
    }
}

