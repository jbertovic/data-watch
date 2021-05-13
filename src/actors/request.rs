use crate::actors::{DataResponse, RequestSchedule, Refresh, ResponseAction, RequestType};
use std::collections::HashMap;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use std::time::Duration;
use async_trait::async_trait;
use xactor::*;
use jmespatch::Expression;
use log::{debug, info};
use http_types::mime;
use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

/// Creates a basic request that runs on a schedule and broadcasts `data_ts`

/// RequestJson
/// Start - nothing on start
/// 
/// <RequestSchedule>
/// - store state for source_name, http client, translation of data
/// - run request and broadcast output message
/// - set interval to send <Refresh>
/// 
/// <Refresh>
/// - run request
/// - broadcast output message
///
/// <Ping>

// IDEA: if things get slow, can we share a client amongst the actors or send the request to a separate broker who handles requests
// IDEA: or store the client state.  currently the request is rebuilt each time

pub struct RequestJson {
    translation: Expression<'static>,
    request_description: RequestSchedule,
}

#[async_trait]
impl Actor for RequestJson {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        // optional: do stuff on handler startup, like subscribing to a Broker
        // ctx.subscribe::<RequestSchedule>().await?;
        debug!("Actor::ReqBasic started");
        Ok(())
    }
}

#[async_trait]
impl Handler<RequestSchedule> for RequestJson {
    async fn handle(&mut self, ctx: &mut Context<Self>, msg: RequestSchedule) {
        debug!("<RequestSchedule> received: {:?}", msg);
        info!("<RequestSchedule> received: {}", msg.source_name);
        self.run_request().await;
        ctx.send_interval(Refresh{}, Duration::from_secs(msg.interval_sec));
    }
}

#[async_trait]
impl Handler<Refresh> for RequestJson {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: Refresh) {
        info!("<Refresh> received:");
        self.run_request().await;
    }
}

impl RequestJson {

    pub fn new(request_description: RequestSchedule) -> Self {

        let translation = jmespatch::compile(request_description.jmespatch_query.as_ref()).unwrap();

        RequestJson {
            translation,
            request_description,
        }
    }

    /// use http client to make request
    async fn run_request(&mut self) {

        // build body if it exists and attach
        // set mime type to form
        // swap variables in for [[ ]]
        let api_url = self.replace_variable(&self.request_description.api_url, false);

        let request = match self.request_description.request_type {
            RequestType::GET => surf::get(api_url),
            RequestType::POST => {
                let body = match self.request_description.body.clone() {
                    Some(b) => self.replace_variable(&b, true),
                    None => String::from(""),
                };
                surf::post(api_url).body(body).content_type(mime::FORM)
            }
        };

        let response = request.recv_string().await.unwrap();

        debug!("Response received: {:?}", &response);

        match &self.request_description.response_action {
            ResponseAction::PUBLISHDATA => {
                let data_received = self.parse_json(&response);
                self.publish_data(data_received).await;
            },
            ResponseAction::STOREVARIABLE => {
                self.store_variable(&response);
            }
        }
    }

    /// parse raw json into Hashmap of data
    /// 
    /// jmespath parse returns: 
    /// 
    /// Multiple measures in one query (includes multiple measure types)- NOT IMPLEMENTED
    /// [
    ///     { measure_name: "", measure_data: {measure_desc1: measure_value1, measure_desc2: measure_value1} },
    ///     { measure_name: "", measure_data: {measure_desc1: measure_value1, measure_desc2: measure_value2} },
    /// ]
    /// 
    /// OR Single measure in one query (includes multiple measure types)
    /// { measure_name: "", measure_data: {measure_desc1: measure_value1, measure_desc2: measure_value2} }
    /// 
    /// 
    /// Return should be Hashmap<String, Vec<(String, f64)>
    /// <measure_name, Vec<measure_desc, measure_value)>>
    /// 
    fn parse_json(&self, json_response: &str) -> HashMap<String, Vec<(String, f64)>> {
        let parsed_json = jmespatch::Variable::from_json(json_response).unwrap();
        let result = self.translation.search(parsed_json).unwrap();
        let mut out = HashMap::new();
        debug!("Parsed result: {:?}", result);
        // decide if array or object (ie multiple measures or one measure with multiple data descriptions)
        if result.is_object() {
            let measure_name = result.as_object().unwrap().get("measure_name").unwrap().as_string().unwrap().to_owned();
            let mut data_points = Vec::new();
            for entry in result.as_object().unwrap().get("measure_data").unwrap().as_object().unwrap() {
                data_points.push((
                    entry.0.to_owned(),
                    entry.1.as_number().unwrap(),
                ))
            }
            out.insert(measure_name, data_points);
        }
        else if result.is_array() {
            unimplemented!();
        }
        out
    }

    async fn publish_data(&self, data_response: HashMap<String, Vec<(String, f64)>>) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut broker = Broker::from_registry().await.unwrap();
        for entry in data_response{
            let measure_name = entry.0;
            for data in entry.1 {
                broker.publish(
                    DataResponse{
                        source_name: self.request_description.source_name.to_owned(),
                        measure_name: measure_name.to_owned(),
                        measure_desc: data.0,
                        measure_value: data.1,
                        timestamp,
                    }
                ).unwrap();
            }
        }
    }

    /// parse raw json into an array of [str, str] to be then owned by shared program variables
    /// in self.storage_var
    /// 
    /// Parsed format:
    /// { "name1": "data1", "name2": "data2" }

    fn store_variable(&self, json_response: &str) {
        let parsed_json = jmespatch::Variable::from_json(json_response).unwrap();
        let result = self.translation.search(parsed_json).unwrap();
        debug!("Parsed result: {:?}", result);
        if result.is_object() {
            let mut storage = self.request_description.storage_var.write().unwrap();
            for entry in result.as_object().unwrap() {
                storage.insert(entry.0.to_owned(), entry.1.as_string().unwrap().to_owned());
            }
        }
    }

    // TODO: Variable needs to be urlencoded or at least have the option if placed in url query or body
    fn replace_variable(&self, text: &str, encode: bool) -> String {
        // locate any global variable placeholder [[ ]] and find variable name
        // replace if variable exists
        let mut newtext = text.to_owned();
        if let Some(start) = text.find("[[") {
            if let Some(end) = text.find("]]") {
                if start<end {
                    let variable = self.request_description.storage_var.read().unwrap();
                    if let Some(replaceto) = variable.get(&text[start+2..end]) {
                        newtext = match encode {
                            true => text.replace(&text[start..end+2], &percent_encode(replaceto.as_bytes(), NON_ALPHANUMERIC).to_string()),
                            false => text.replace(&text[start..end+2], replaceto)
                        };
                    }
                    else {
                        newtext = text.replace(&text[start..end+2], "");
                    }
                }
            }
        }
        newtext
    }
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;
    use std::sync::Arc;
    use super::*;

    #[test]
    fn json_parsing_variables() {
        let json_raw = r#" { "variable_name": "name", "variable_data": "data" } "#;
        let request = RequestJson::new(
            RequestSchedule {
                source_name: String::from("Test"),
                api_url: String::new(),
                request_type: RequestType::GET,
                body: None,
                interval_sec: 0,
                jmespatch_query: String::from("{ variable_data: variable_data, variable_name: variable_name }"),
                storage_var: Arc::new(RwLock::new(HashMap::new())),
                response_action: ResponseAction::STOREVARIABLE,
            }
        );
        request.store_variable(json_raw);
        {
            let reader = request.request_description.storage_var.read().unwrap();
            assert_eq!(reader.get("variable_name").unwrap(), "name");
            assert_eq!(reader.get("variable_data").unwrap(), "data");
        }
    }

    #[test]
    fn json_parsing_single_dataresponse() {
        // OR Single measure in one query (includes multiple measure types)
        // { measure_name: "", measure_data: {measure_desc1: measure_value1, measure_desc2: measure_value2} }
        // Return should be Hashmap<string, Vec<(String, f64)>
        // <measure_name, Vec<measure_desc, measure_value)>>
        let json_raw = r#" 
        { 
            "measure_name": "name", 
            "measure_data": 
            {
                "desc1": 1.0,
                "desc2": 2.0
            }
        } "#;

        let request = RequestJson::new(
            RequestSchedule {
                source_name: String::from("Test2"),
                api_url: String::new(),
                request_type: RequestType::GET,
                body: None,
                interval_sec: 0,
                jmespatch_query: String::from("{measure_name: measure_name, measure_data: measure_data}"),
                storage_var: Arc::new(RwLock::new(HashMap::new())),
                response_action: ResponseAction::PUBLISHDATA,
            }
        );

        let datahash = request.parse_json(&json_raw);
        assert_eq!(datahash.get(&String::from("name")).unwrap(), &vec!((String::from("desc1"), 1.0 as f64), (String::from("desc2"), 2.0 as f64)));
    }

}
