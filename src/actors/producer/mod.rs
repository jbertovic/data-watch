mod webrequest;
mod producer;

pub use webrequest::WebProducer as WebProducer;

/// Defines the type of action on Producer
#[derive(Debug, Clone)]
pub enum ProducerAction {
    PUBLISHDATA,
    STOREVARIABLE,
}

// Defines type of Web Request
#[derive(Debug, Clone)]
pub enum WebRequestType {
    GET,
    POST,
}
