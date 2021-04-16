/// Write to csv file and use name of measure for filename
/// If file exists append to existing file
/// Path is held in state
/// 



//  TODO: Have a file name created for name of measure set on initiation of actor
//   - don't try to do this with every measure to store or you can combine measures to store
//   - maybe keep a vector of names to store in file
//   - or create a unique routing name from name+description?

use async_trait::async_trait;
use async_std::{fs::File, io::{BufWriter, prelude::WriteExt}};
use xactor::*;
use super::messages::DataResponse;
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


pub struct CsvWriter {
    writer: BufWriter<File>,
}

impl CsvWriter {
    pub fn new(writer: BufWriter<File>) -> CsvWriter {
        CsvWriter {
            writer: writer,
        }
    }
}

impl Default for CsvWriter {
   fn default() ->  CsvWriter {
        async_std::task::block_on(
            async {
                let file = File::create("data.csv").await.unwrap();
                CsvWriter {
                    writer: BufWriter::new(file),
                } 
            }
        )
    }
}

#[async_trait]
impl Actor for CsvWriter {
    async fn started(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        info!("CsvWriter started");
        ctx.subscribe::<DataResponse>().await?;
        Ok(())
    }
}

#[async_trait]
impl Handler<DataResponse> for CsvWriter {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: DataResponse) {
        // append to csv file stream
        self.writer.write(format!("\"{}\", \"{}\", \"{}\", {}, {}\n", 
            msg.source_name, 
            msg.data_ts.0,
            msg.data_ts.1,
            msg.data_ts.2,
            msg.data_ts.3
        ).as_bytes()).await.unwrap();
        self.writer.flush().await.unwrap();
    }
}

