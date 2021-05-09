use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;

pub mod actors;

pub type SharedVar = Arc<RwLock<HashMap<String, String>>>;

// Example data structures
// datasource: weather, name: houston, desc: temp, value: 80.0
// datasource: TDAPI, name: TRP, desc: last, value: 51.00


//EXAMPLES
// TODO: Weather - store variable in memory cache
// TODO: Weather - use variable from memory cache in path of url
// TODO: Quote - get token authorization from refresh key and store into memory cache
// TODO: Quote - get quotes for a series of stocks
// TODO: Configuration can be saved and loaded - example combining Weather and Quote

//PROGRAM
// TODO: create universal variables to store in memory cache
// TODO: include a way to translate URL to combine variables in path
// TODO: add memory cache for measures that is live over a finite time, able to register which variables
// TODO: convert measure (function) or combination of measures to calculated new measure
// TODO: datastore to csv (use state for filename) - specify which measures and which file
// TODO: program initiation structure to keep track of actors to minimize users of library having to initialize everything
// TODO: an elegant way to exit the program

// FUTURE: datastore to postgres database
// FUTURE: add server to manage scheduler
// FUTURE: register alert to watch data stream
// FUTURE: react to alert
// FUTURE: add ability to fetch events/transactions that aren't timestamp related


// Playground links
// ace<rwlock<hashmap>>
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=91cfd63c6b34d2ecc527dd1a4e2f95e1
// replace variable
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b51dec5bba77001ffa8d946911d5c529

