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

// Example that grabs current weather from openweathermaps.org
// API key is stored in shared varibles and grabbed from Environment Variable WEATHER_KEY

#[async_std::main]
async fn main() -> Result<(), xactor::Error> {
    env_logger::init();

    let shared_variables: SharedVar = Arc::new(RwLock::new(HashMap::new()));

    let key = env::var("WEATHER_KEY").expect("Need API key from https://openweathermaps.org");

    // store global variables - usually API keys
    {
        let mut storage = shared_variables.write().unwrap();
        storage.insert(String::from("WEATHER_KEY"), key);
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

    // Build Request
    let request_message = WebProducerSchedule {
        source_name: String::from("Weather"), 
        api_url: String::from("https://api.openweathermap.org/data/2.5/weather?q=Houston&units=imperial&appid=[[WEATHER_KEY]]"), 
        request_type: ApiRequestType::GET,
        body: None,
        header: None,
        //                   sec min hour dayofmonth month  dayofweek
        cron: String::from("0/10  *  *  *  *  *"),
        jmespatch_query: String::from("merge({measure_name: name},{measure_data: main})"), 
        storage_var: shared_variables.clone(),
        response_action: ProducerAction::PUBLISHDATA,
    };

    // Send Request to scheduler
    scheduler_addr.send(request_message)?;

    task::sleep(Duration::from_secs(60 * 5)).await;

    scheduler_addr.send(Stop)?;

    scheduler_task.await;
    Ok(())
}
