# Developing in Real-time.  Not a useable product!

Thoughts around building an Data Watch framework in Rust that would allow me to stream data from in multiple ways that would used for alerts, storage, or viewing. Start with building

Setup using an actor framework.


## Starting Project

- Able to define a way to extract different API endpoints into real-time data.
- Develop ways to consume this data for different purposes.
- Use a single Data Type to categorize a measure - start with f64.
- Allow flexibility by holding global store of variables to pass to requests and for other purposes.

Completed project should be able to (P)roduce data and (C)onsume data.  In an actor framework; (P)roducer will publish data and (C)onsumer will subscribe to data.

## First Product - Actor and Message setup

```
User/Config SENDS (P)roducerRequest TO Scheduler(actor) 

Scheduler(actor) SEND (P)roducerRequest(message) TO Producer(actor)

Producer(actor) SEND Refresh(message) TO Producer(actor): based on schedule
Producer(actor) ACTION: PUBLISH DataResponse(message) OR STOREVARIABLE
```

## Data format

```
pub struct DataResponse{
    pub source_name: String,
    pub measure_name: String,
    pub measure_desc: String,
    pub measure_value: f64,
    pub timestamp: u64 
}
```

## Producers

- API JSON response request

## Consumers
- Stdout print data
- CSV data storage
- Memory cache
- Alert criteria watch
- DB storage





