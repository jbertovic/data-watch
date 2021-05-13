use std::collections::HashMap;
use std::sync::{RwLock, Arc};
use std::time::Duration;
use std::env;
use xactor::Actor;
use async_std::task;
use data_watch::actors::{StdoutWriter, RequestSchedule, Scheduler, Stop, ResponseAction, RequestType};
use data_watch::SharedVar;

// Example that grabs current quotes from tdameritrade's api using current token
// Example includes using refresh token to initialize a new token that is renewed every 30 min
// A valid refresh token is stored in Environment Variable TDREFRESHTOKEN and TDCLIENTID
// Example uses both a GET request for the quotes and a POST request for refreshing new Token
// 
// API documentation at https://developer.tdameritrade.com/

#[async_std::main]
async fn main() -> Result<(), xactor::Error> {

    env_logger::init();
    
    let shared_variables: SharedVar = Arc::new(RwLock::new(HashMap::new()));
    
    let refresh_token = env::var("TDREFRESHTOKEN")
        .expect("Need Refresh Token for TDAmeritrade");
    // let client_id = env::var("TDCLIENTID")
    //     .expect("Need TD Client ID for TDAmeritrade");
    
    // store global variables - usually API keys
    {
        let mut storage = shared_variables.write().unwrap();
        storage.insert(String::from("TDREFRESHTOKEN"), refresh_token);
        // storage.insert(String::from("TDCLIENTID"), client_id);
    }

    // start scheduler
    let scheduler = Scheduler::new().start().await?;
    let scheduler_addr = scheduler.clone();

    // send scheduler clone to watch for shutdown
    let scheduler_task = xactor::spawn(async {
        scheduler.wait_for_stop().await;
    });

    // start datawriter to push output to screen
    let _datawriter = StdoutWriter.start().await?;

    // start csvwriter to push output to csv file
    // let _csvwriter = CsvWriter::default().start().await?;

    // example using POST request configuration and response_action into variable
    // TODO: need to add header or body and request type: GET / POST

    // Build Request 
    let request_message = RequestSchedule{ 
        source_name: String::from("TD_AUTH"), 
        api_url: String::from("https://api.tdameritrade.com/v1/oauth2/token"), 
        request_type: RequestType::POST,
        body: Some(String::from("grant_type=refresh_token&refresh_token=[[TDREFRESHTOKEN]]&client_id=J3ROAVSNNFTLC9RJE4BD2DO2WJ9JE4DG")),
        interval_sec: 1700,
        jmespatch_query: String::from("{ TDTOKEN: access_token }"), 
        storage_var: shared_variables.clone(),
        response_action: ResponseAction::STOREVARIABLE,
    };

    // Send Request to scheduler
    scheduler_addr.send(request_message)?;

    task::sleep(Duration::from_secs(10)).await;

    // print global variables 
    {
        let reader = shared_variables.read().unwrap();
        println!("{:?}", reader);
    }

    scheduler_addr.send(Stop)?;    

    scheduler_task.await;
    Ok(())
}
