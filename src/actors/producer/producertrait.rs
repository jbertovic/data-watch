use async_trait::async_trait;

#[async_trait]
pub trait ProducerTrait<T> {
    async fn run_request() -> String;
    fn translate(response: String) -> T; 
    async fn response_action(&self, data: T);
}

// T should be key/value or 
// T should be HashMap<String, Vec<(String, f64)>> (What's this type?)

//https://stackoverflow.com/questions/53085270/how-do-i-implement-a-trait-with-a-generic-method

