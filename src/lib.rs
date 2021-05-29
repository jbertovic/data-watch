use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;

pub mod actors;


// Utility
// Collection of functions to store variables in shared storage, parse json to match data model, and 
// read stored variables to update strings 
pub mod utility;

// global variables that can be used to implement in Producer configuration
pub type SharedVar = Arc<RwLock<HashMap<String, String>>>;

// POSSIBLE:
// create generic producer
// move web items out of actor to run


// TODO: schedule defined using cron formatting
// TODO: can we stream cron dates instead of just iterating?
// TODO: selection trait on data to be used in different consumers
// TODO: update file consumer (file or directory structure / 
        // action to define appending or latest / format csv, json, others?)
// TODO: add memory cache actor for measures that is live over a finite time, able to register which variables
// TODO: Selective printing or csv storage
// TODO: Configuration can be saved and loaded - example combining Weather and Quote
// TODO: Alert consumer that matches criteria
// TODO: Need error checking
// TODO: convert measure (function) or combination of measures to calculated new measure - New Producer
// TODO: to database consumer
// TODO: program initiation structure to keep track of actors to minimize users of library having to initialize everything
// TODO: an elegant way to exit the program
// TODO: a way to interact with the program either through CLI or server or both
// TODO: better Error handling - think about all the things that could go wrong and create error type or anyhow
// TODO: better way to deal with dates  (maybe a utility folder with utility.rs -> parsing, dates)

// FUTURE: Add an ability to push items to data-watch from outside
// FUTURE: add server to manage scheduler
// FUTURE: (NOT sure if this belongs) add ability to fetch events/transactions that aren't timestamp related


// Playground links
// ace<rwlock<hashmap>>
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=91cfd63c6b34d2ecc527dd1a4e2f95e1
// replace variable
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b51dec5bba77001ffa8d946911d5c529

