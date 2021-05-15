use std::collections::HashMap;
use log::debug;
use crate::SharedVar;
use percent_encoding::{NON_ALPHANUMERIC, percent_encode};


/// Parse JSON to HashMap of DataResponse format using a jmespath expression
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
pub fn parse_json(expression: &jmespatch::Expression<'static>, json_response: &str) -> HashMap<String, Vec<(String, f64)>> {
    let parsed_json = jmespatch::Variable::from_json(json_response).unwrap();
    let result = expression.search(parsed_json).unwrap();
    let mut out = HashMap::new();
    debug!("Parsed result: {:?}", result);
    // decide if array or object (ie multiple measures or one measure with multiple data descriptions)
    if result.is_object() {
        let parsed = parse_one_measure(&result);
        out.insert(parsed.0, parsed.1);
    }
    else if result.is_array() {
        for each_result in result.as_array().unwrap() {
            let parsed = parse_one_measure(&each_result);
            out.insert(parsed.0, parsed.1);
        }
    }
    out
}

fn parse_one_measure(result: &jmespatch::Variable) -> (String, Vec<(String, f64)>) {
    let measure_name = result.as_object().unwrap().get("measure_name").unwrap().as_string().unwrap().to_owned();
    let mut data_points = Vec::new();
    for entry in result.as_object().unwrap().get("measure_data").unwrap().as_object().unwrap() {
        data_points.push((
            entry.0.to_owned(),
            entry.1.as_number().unwrap(),
        ))
    }
    (measure_name, data_points)
}

/// Update string to replace [[VARIABLE]] with a variable stored in shared variables
/// Created as recursive funciton to add multiple variables to one string
/// If variable is not found it will replace [[VARIABLE]] with ""
/// 
/// encode option will urlencode the variable if set to true
/// 
pub fn swap_variable(storage_var: &SharedVar, text: &str, encode: bool) -> String {
    let mut newtext = text.to_owned();
    if let Some(start) = text.find("[[") {
        if let Some(end) = text.find("]]") {
            if start<end {
                let variable = storage_var.read().unwrap();
                if let Some(replaceto) = variable.get(&text[start+2..end]) {
                    newtext = match encode {
                        true => text.replace(&text[start..end+2], &percent_encode(replaceto.as_bytes(), NON_ALPHANUMERIC).to_string()),
                        false => text.replace(&text[start..end+2], replaceto)
                    };
                }
                else {
                    newtext = text.replace(&text[start..end+2], "");
                }
                newtext = swap_variable(&storage_var, newtext.as_ref(), encode);
            }
        }
    }
    newtext
}


/// parse raw json into an array of [str, str] to be then inserted into shared variables
/// in `SharedVar` format
/// 
/// Parsed format:
/// { "name1": "data1", "name2": "data2" }
pub fn store_variable(expression: &jmespatch::Expression<'static>, storage_var: &SharedVar, json_response: &str) {
    let parsed_json = jmespatch::Variable::from_json(json_response).unwrap();
    let result = expression.search(parsed_json).unwrap();
    debug!("Parsed result: {:?}", result);
    if result.is_object() {
        let mut storage = storage_var.write().unwrap();
        for entry in result.as_object().unwrap() {
            storage.insert(entry.0.to_owned(), entry.1.as_string().unwrap().to_owned());
        }
    }
}



#[cfg(test)]
mod tests {
    use std::sync::RwLock;
    use std::sync::Arc;
    use super::*;

    #[test]
    fn json_parsing_to_store_shared_variables() {
        let json_raw = r#" { "variable_name": "name", "variable_data": "data" } "#;
        let expression = jmespatch::compile("{ variable_data: variable_data, variable_name: variable_name }").unwrap();
        let storage_var = Arc::new(RwLock::new(HashMap::new()));
        store_variable(&expression, &storage_var, json_raw);
        {
            let reader = storage_var.read().unwrap();
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
        let expression = jmespatch::compile("@").unwrap();
        let datahash = parse_json(&expression, &json_raw);
        assert_eq!(datahash.get(&String::from("name")).unwrap(), &vec!((String::from("desc1"), 1.0 as f64), (String::from("desc2"), 2.0 as f64)));
    }

    #[test]
    fn json_parsing_multiple_dataresponse() {
        // OR Single measure in one query (includes multiple measure types)
        // { measure_name: "", measure_data: {measure_desc1: measure_value1, measure_desc2: measure_value2} }
        // Return should be Hashmap<string, Vec<(String, f64)>
        // <measure_name, Vec<measure_desc, measure_value)>>
        let json_raw = r#" 
        [
            { 
                "measure_name": "name1", 
                "measure_data": 
                {
                    "desc1": 1.0,
                    "desc2": 2.0
                }
            },
            { 
                "measure_name": "name2", 
                "measure_data": 
                {
                    "desc1": 3.0,
                    "desc2": 4.0
                }
            }
        ] "#;

        let expression = jmespatch::compile("@").unwrap();
        let datahash = parse_json(&expression, &json_raw);
        assert_eq!(datahash.get(&String::from("name1")).unwrap(), &vec!((String::from("desc1"), 1.0 as f64), (String::from("desc2"), 2.0 as f64)));
        assert_eq!(datahash.get(&String::from("name2")).unwrap(), &vec!((String::from("desc1"), 3.0 as f64), (String::from("desc2"), 4.0 as f64)));
    }




    #[test]
    fn swap_one_variable() {
        let text_raw = "Text string looking to swap [[ONE]] Variable.";
        let storage_var = Arc::new(RwLock::new(HashMap::new()));
        {
            let mut w = storage_var.write().unwrap();
            w.insert(String::from("ONE"), String::from("1"));
        }
        assert_eq!(String::from("Text string looking to swap 1 Variable."), swap_variable(&storage_var, text_raw, false));
    }

    #[test]
    fn swap_more_than_one_variable() {
        let text_raw = "Swap [[ONE]] Variable and [[TWO]] Variables and [[NO]]no Variable.";
        let storage_var = Arc::new(RwLock::new(HashMap::new()));
        {
            let mut w = storage_var.write().unwrap();
            w.insert(String::from("ONE"), String::from("1"));
            w.insert(String::from("TWO"), String::from("2"));
        }
        let new_text = swap_variable(&storage_var, text_raw, false);
        assert_eq!(String::from("Swap 1 Variable and 2 Variables and no Variable."), new_text);
    }

}