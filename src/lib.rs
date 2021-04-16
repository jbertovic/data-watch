pub mod actors;

/// (name, value description, value, epoch time)
#[derive(Debug, Clone)]
pub struct DataTimeStamp(pub String, pub String, pub f64, pub u64);

// TODO: Handle authorization flow and create ReqAuth
// TODO: add memory cache for measures that is live over a finite time (24hrs?)
// TODO: convert measure or combination of measures to calculated new measure
// TODO: datastore to csv (use state for filename)
// TODO: datastore to postgres database

// FUTURE: add server to manage scheduler
// FUTURE: register alert to watch data stream
// FUTURE: react to alert
// FUTURE: add ability to fetch events/transactions that aren't timestamp related

