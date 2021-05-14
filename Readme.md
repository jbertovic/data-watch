# Developing in Real-time.  Not a useable product!

Thoughts around building an Data Watch framework in Rust that would allow me to stream data from multiple API that would used for alerts, storage, or viewing.

Setup using an actor framework.


# Starting Project

1) Develop a generic way to descript API extraction requests and a data structure to return.

2) Building out a scheduler to deal with managing multiple API endpoints on an interval

3) Ability of pass a transpormation function that allows response to be converted to time series data

4) Listen for clean data for storage in memory or store on disk/db


# First Product - Actor and Message setup

```
User/Config SENDS RequestJson(message) TO Scheduler(actor) 

Scheduler(actor) SEND RequestJson(message) TO RequestJson(actor)

RequestJson(actor) SEND Refresh(message) TO RequestJson(actor)
RequestJson(actor) RequestAction: PUBLISH DataResponse(message) OR STOREVARIABLE
```






