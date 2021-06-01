mod apirequest;
mod producertrait;
mod publishdata;
mod webproducer;

pub use webproducer::WebProducer;

/// Defines the type of action on Producer
#[derive(Debug, Clone)]
pub enum ProducerAction {
    PUBLISHDATA,
    STOREVARIABLE,
}

// Defines type of Web Request
#[derive(Debug, Clone)]
pub enum ApiRequestType {
    GET,
    POST,
}
