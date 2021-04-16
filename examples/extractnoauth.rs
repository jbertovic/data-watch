
use std::time::Duration;
use std::env;
use xactor::Actor;
use async_std::task;
use data_watch::actors::{CsvWriter, StdoutWriter, RequestSchedule, Scheduler};
use data_watch::DataTimeStamp;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};


#[async_std::main]
async fn main() -> Result<(), xactor::Error> {

    env_logger::init();
    
    let address = env::var("ETHPUBADDRESS")
        .expect("Ethereum public addressis missing in env variable ETHPUBADDRESS");
    let api_url = format!("{}{}", API, address);
    
    // start scheduler
    let scheduler = Scheduler::new().start().await?;

    // start datawriter to push output to screen
    let _datawriter = StdoutWriter.start().await?;
    let _csvwriter = CsvWriter::default().start().await?;

    // start interval at 10sec
    let request_message = RequestSchedule{ 
        source_name: String::from("compound_account"), 
        api_url, 
        interval_sec: 60,
        translation: translation, 
    };

    scheduler.send(request_message)?;

    task::sleep(Duration::from_secs(60*5+10)).await;
    Ok(())
}


// Configuration of Request to schedule

const API:&str = "https://api.compound.finance/api/v2/account?addresses[]=";

#[derive(Deserialize, Debug)]
struct CompoundAccounts {
    accounts: Vec<Account>,
}

#[derive(Deserialize, Debug)]
struct Account {
    address: String,
    tokens: Vec<Token>,
}

#[derive(Deserialize, Debug)]
struct Token {
    symbol: String,
    lifetime_supply_interest_accrued: ValueField,
    supply_balance_underlying: ValueField,
}

#[derive(Deserialize, Debug)]
struct ValueField {
    value: String
}


fn translation(data: String) -> Vec<DataTimeStamp>  {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let json: CompoundAccounts = serde_json::from_str(&data).unwrap();
    let mut out = Vec::new();
    for a in &json.accounts {
        for t in &a.tokens {
            out.push(DataTimeStamp(t.symbol.clone(), String::from("accrued"), t.lifetime_supply_interest_accrued.value.parse().unwrap(), timestamp));
            out.push(DataTimeStamp(t.symbol.clone(), String::from("supply"), t.supply_balance_underlying.value.parse().unwrap(), timestamp));
        }
    }
    out
}

