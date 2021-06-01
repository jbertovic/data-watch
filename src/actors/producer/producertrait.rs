use async_trait::async_trait;

#[async_trait]
pub trait ProducerTrait<T> {
    async fn run_request() -> String;
    fn translate(response: String) -> T; 
    async fn response_action(&self, data: T);
}

// T should be key/value or 
// T should be HashMap<String, Vec<(String, f64)>> (What's this type?)