mod messages;
mod scheduler;
mod request;
mod stdoutwriter;
mod csvwriter;

pub use messages::RequestSchedule as RequestSchedule;
pub use messages::DataResponse as DataResponse;
pub use messages::Refresh as Refresh;
pub use messages::Stop as Stop;

pub use scheduler::Scheduler as Scheduler;
pub use request::RequestJson as RequestJson;
pub use stdoutwriter::StdoutWriter as StdoutWriter;
pub use csvwriter::CsvWriter as CsvWriter;


/// Defines the type of action on any responses received from requests
#[derive(Debug, Clone)]
pub enum ResponseAction {
    PUBLISHDATA,
    STOREVARIABLE,
}

#[derive(Debug, Clone)]
pub enum RequestType {
    GET,
    POST,
}