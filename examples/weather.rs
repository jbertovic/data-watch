
use std::time::Duration;
use std::env;
use xactor::Actor;
use async_std::task;
use data_watch::actors::{CsvWriter, StdoutWriter, RequestSchedule, Scheduler, Stop};
use data_watch::DataTimeStamp;
use std::time::{SystemTime, UNIX_EPOCH};

/// Example that grabs current weather from openweathermaps.org
/// 
#[async_std::main]
async fn main() -> Result<(), xactor::Error> {

    env_logger::init();
    
    let key = env::var("WEATHER_KEY")
        .expect("Need API key from https://openweathermaps.org");
    let api_url = format!("{}{}", API, key);
    
    // start scheduler
    let scheduler = Scheduler::new().start().await?;
    let scheduler_addr = scheduler.clone();

    let scheduler_task = xactor::spawn(async {
        scheduler.wait_for_stop().await;
    });

    // start datawriter to push output to screen
    let _datawriter = StdoutWriter.start().await?;
    let _csvwriter = CsvWriter::default().start().await?;

    // start interval at 10sec
    let request_message = RequestSchedule{ 
        source_name: String::from("Weather"), 
        api_url, 
        interval_sec: 60*5,
        translation: translation, 
    };

    scheduler_addr.send(request_message)?;

    task::sleep(Duration::from_secs(10+1)).await;

    scheduler_addr.send(Stop)?;    

    scheduler_task.await;
    Ok(())
}


// Configuration of Request to schedule

const API:&str = "https://api.openweathermap.org/data/2.5/weather?q=Houston&units=imperial&appid=";

fn translation(data: String) -> Vec<DataTimeStamp>  {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let json: serde_json::Value = serde_json::from_str(&data).unwrap();
    let mut out = Vec::new();
    out.push(DataTimeStamp(String::from("Houston"), String::from("Temp"), json["main"]["temp"].as_f64().unwrap(), timestamp));
    out.push(DataTimeStamp(String::from("Houston"), String::from("Humidity"), json["main"]["humidity"].as_f64().unwrap(), timestamp));
    out.push(DataTimeStamp(String::from("Houston"), String::from("Humidity"), json["main"]["pressure"].as_f64().unwrap(), timestamp));
    out
}

