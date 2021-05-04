pub mod actors;

/// (name, value description, value, epoch time)
#[derive(Debug, Clone)]
pub struct DataTimeStamp(pub String, pub String, pub f64, pub u64);


//IMPORTANT
// data structure on DataTimeStamp - do i need source and name?
// datasource: weather, name: houston, desc: temp, value: 80.0
// name: weather-houston, desc: temp, value: 80.0
// 
// datasource: TDAPI, name: TRP, desc: last, value: 51.00
// name: quote, desc: TRP-last, value: 51.00


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


// FUTURE: datastore to postgres database
// FUTURE: add server to manage scheduler
// FUTURE: register alert to watch data stream
// FUTURE: react to alert
// FUTURE: add ability to fetch events/transactions that aren't timestamp related

