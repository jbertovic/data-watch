use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;

pub mod actors;


// Utility
// Collection of functions to store variables in shared storage, parse json to match data model, and 
// read stored variables to update strings 
pub mod utility;

pub type SharedVar = Arc<RwLock<HashMap<String, String>>>;

// Example data structures
// datasource: weather, name: houston, desc: temp, value: 80.0
// datasource: TDAPI, name: TRP, desc: last, value: 51.00


// EXAMPLES
// TODO: schedule defined using cron formatting
// TODO: Weather,Crypto,Stocks - store variable in memory cache 
// TODO: Selective printing or csv storage
// TODO: Configuration can be saved and loaded - example combining Weather and Quote
// TODO: Alert consumer that matches criteria

//PROGRAM
// TODO: add memory cache actor for measures that is live over a finite time, able to register which variables
// TODO: convert measure (function) or combination of measures to calculated new measure - New Producer
// TODO: datastore to csv (use state for filename) - specify which measures and which file - NOT ALL measures
// TODO: program initiation structure to keep track of actors to minimize users of library having to initialize everything
// TODO: an elegant way to exit the program
// TODO: a way to interact with the program either through CLI or server or both
// TODO: better Error handling - think about all the things that could go wrong and create error type or anyhow
// TODO: better way to deal with dates  (maybe a utility folder with utility.rs -> parsing, dates)

// FUTURE: datastore to postgres database
// FUTURE: Add an ability to push items to data-watch from outside
// FUTURE: add server to manage scheduler
// FUTURE: register alert to watch data stream
// FUTURE: react to alert
// FUTURE: (NOT sure if this belongs) add ability to fetch events/transactions that aren't timestamp related


// Playground links
// ace<rwlock<hashmap>>
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=91cfd63c6b34d2ecc527dd1a4e2f95e1
// replace variable
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b51dec5bba77001ffa8d946911d5c529

