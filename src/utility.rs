

fn parse_json(&self, json_response: &str) -> HashMap<String, Vec<(String, f64)>> {
    let parsed_json = jmespatch::Variable::from_json(json_response).unwrap();
    let result = self.translation.as_ref().unwrap().search(parsed_json).unwrap();
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

fn replace_variable(&self, text: &str) -> String {
    // locate any global variable placeholder [[ ]] and find variable name
    // replace if variable exists
    let mut newtext = text.to_owned();
    if let Some(start) = text.find("[[") {
        if let Some(end) = text.find("]]") {
            if start<end {
                let variable = self.storage_var.as_ref().unwrap().read().unwrap();
                if let Some(replaceto) = variable.get(&text[start+2..end]) {
                    newtext = text.replace(&text[start..end+2], replaceto);
                }
                else {
                    newtext = text.replace(&text[start..end+2], "");
                }
            }
        }
    }
    newtext
}


#[cfg(test)]
mod tests {
    use std::sync::RwLock;
    use std::sync::Arc;
    use super::*;

    #[test]
    fn json_parsing_variables() {
        let json_raw = r#" { "variable_name": "name", "variable_data": "data" } "#;
        let request = RequestJson {
            source_name: String::from("Test"),
            request: None,
            translation: Some(jmespatch::compile("{ variable_data: variable_data, variable_name: variable_name }").unwrap()),
            storage_var: Some(Arc::new(RwLock::new(HashMap::new()))),
            response_action: None,
        };
        request.store_variable(json_raw);
        {
            let reader = request.storage_var.as_ref().unwrap().read().unwrap();
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
        let request = RequestJson {
            source_name: String::from("Test2"),
            request: None,
            translation: Some(jmespatch::compile("{measure_name: measure_name, measure_data: measure_data}").unwrap()),
            storage_var: None,
            response_action: None,
        };

        let datahash = request.parse_json(&json_raw);
        assert_eq!(datahash.get(&String::from("name")).unwrap(), &vec!((String::from("desc1"), 1.0 as f64), (String::from("desc2"), 2.0 as f64)));
    }

}
