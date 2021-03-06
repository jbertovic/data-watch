use async_std::task;
use data_watch::actors::consumer::StdoutConsumer;
use data_watch::actors::messages::{Stop, WebProducerSchedule};
use data_watch::actors::producer::{ApiRequestType, ProducerAction};
use data_watch::actors::Scheduler;
use data_watch::SharedVar;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use xactor::Actor;

// Example that grabs current quotes from coinbase api for Bitcoin, Ethureum and Compound
// Also includes data for compound Defi held in wallet ata specified Wallet public address
//
// Coinbase API for BTC: https://api.pro.coinbase.com/products/BTC-USD/ticker
// Coinbase API for ETH: https://api.pro.coinbase.com/products/ETH-USD/ticker
// Coinbase API for COMP: https://api.pro.coinbase.com/products/COMP-USD/ticker
// Compound API: https://api.compound.finance/api/v2/account?addresses[]=[[ETHPUBADDRESS]]

#[async_std::main]
async fn main() -> Result<(), xactor::Error> {
    env_logger::init();

    let shared_variables: SharedVar = Arc::new(RwLock::new(HashMap::new()));

    let address = env::var("ETHPUBADDRESS")
        .expect("Ethereum public addressis missing in env variable ETHPUBADDRESS");

    // store global variables - usually API keys
    {
        let mut storage = shared_variables.write().unwrap();
        storage.insert(String::from("ETHPUBADDRESS"), address);
    }

    // start scheduler
    let scheduler = Scheduler::default().start().await?;
    let scheduler_addr = scheduler.clone();

    // send scheduler clone to watch for shutdown
    let scheduler_task = xactor::spawn(async {
        scheduler.wait_for_stop().await;
    });

    // start datawriter to push output to screen
    let _datawriter = StdoutConsumer.start().await?;

    // start csvwriter to push output to csv file
    // let _csvwriter = CsvWriter::default().start().await?;

    // Build Request to Retreive Crypto Currency prices from coinbase
    let coinbase1 = WebProducerSchedule {
        source_name: String::from("COINBASE_PRO"),
        api_url: String::from("https://api.pro.coinbase.com/products/BTC-USD/ticker"),
        request_type: ApiRequestType::GET,
        body: None,
        header: None,
        //                   sec min hour dayofmonth month  dayofweek
        cron: String::from("0  */1  *   *  *  *"),
        jmespatch_query: String::from(
            "merge({ measure_data: {mark: to_number(price)} }, { measure_name: `\"BTC-USD\"`})",
        ),
        storage_var: shared_variables.clone(),
        response_action: ProducerAction::PUBLISHDATA,
    };

    // Send Request to scheduler
    scheduler_addr.send(coinbase1)?;

    // Build Request to Retreive Crypto Currency prices from coinbase
    let coinbase2 = WebProducerSchedule {
        source_name: String::from("COINBASE_PRO"),
        api_url: String::from("https://api.pro.coinbase.com/products/ETH-USD/ticker"),
        request_type: ApiRequestType::GET,
        body: None,
        header: None,
        //                   sec min hour dayofmonth month  dayofweek
        cron: String::from("30  */1  *   *  *  *"),
        jmespatch_query: String::from(
            "merge({ measure_data: {mark: to_number(price)} }, { measure_name: `\"ETH-USD\"`})",
        ),
        storage_var: shared_variables.clone(),
        response_action: ProducerAction::PUBLISHDATA,
    };

    // Send Request to scheduler
    scheduler_addr.send(coinbase2)?;

    // Build Request to Retreive Crypto Currency prices from coinbase
    let coinbase3 = WebProducerSchedule {
        source_name: String::from("COINBASE_PRO"),
        api_url: String::from("https://api.pro.coinbase.com/products/COMP-USD/ticker"),
        request_type: ApiRequestType::GET,
        body: None,
        header: None,
        //                   sec min hour dayofmonth month  dayofweek
        cron: String::from("30  */1  *   *  *  *"),
        jmespatch_query: String::from(
            "merge({ measure_data: {mark: to_number(price)} }, { measure_name: `\"COMP-USD\"`})",
        ),
        storage_var: shared_variables.clone(),
        response_action: ProducerAction::PUBLISHDATA,
    };

    // Send Request to scheduler
    scheduler_addr.send(coinbase3)?;

    // Build Request to Retreive Crypto Currency prices from coinbase
    let compound = WebProducerSchedule {
            source_name: String::from("DEFI_COMPOUND"), 
            api_url: String::from("https://api.compound.finance/api/v2/account?addresses[]=[[ETHPUBADDRESS]]"), 
            request_type: ApiRequestType::GET,
            body: None,
            header: None,
            //                   sec min hour dayofmonth month  dayofweek
            cron: String::from("15  */1  *   *  *  *"),
            jmespatch_query: String::from("accounts[0].tokens[].{measure_name: symbol, measure_data: { balance: to_number(supply_balance_underlying.value), accrued: to_number(lifetime_supply_interest_accrued.value)} }"), 
            storage_var: shared_variables.clone(),
            response_action: ProducerAction::PUBLISHDATA,
        };

    // Send Request to scheduler
    scheduler_addr.send(compound)?;

    // Build Request to Retreive Crypto Currency prices from coinbase
    let compound2 = WebProducerSchedule {
            source_name: String::from("DEFI_COMPOUND"), 
            api_url: String::from("https://api.compound.finance/api/v2/ctoken"), 
            request_type: ApiRequestType::GET,
            body: None,
            header: None,
            //                   sec min hour dayofmonth month  dayofweek
            cron: String::from("45  */1  *   *  *  *"),
            jmespatch_query: String::from("cToken[?symbol==`\"cUSDC\"`||symbol==`\"cDAI\"`].{measure_name: symbol, measure_data: {supply_rate: to_number(supply_rate.value)}}"), 
            storage_var: shared_variables.clone(),
            response_action: ProducerAction::PUBLISHDATA,
        };

    // Send Request to scheduler
    scheduler_addr.send(compound2)?;

    task::sleep(Duration::from_secs(60 * 3)).await;

    scheduler_addr.send(Stop)?;

    scheduler_task.await;
    Ok(())
}
