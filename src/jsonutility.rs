use log::debug;
use std::collections::HashMap;

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
pub fn parse_json_data(
    expression: &jmespatch::Expression<'static>,
    json_response: &str,
) -> HashMap<String, Vec<(String, f64)>> {
    let parsed_json = jmespatch::Variable::from_json(json_response).unwrap();
    let result = expression.search(parsed_json).unwrap();
    let mut out = HashMap::new();
    debug!("Parsed result: {:?}", result);
    // decide if array or object (ie multiple measures or one measure with multiple data descriptions)
    if result.is_object() {
        let parsed = parse_one_measure(&result);
        out.insert(parsed.0, parsed.1);
    } else if result.is_array() {
        for each_result in result.as_array().unwrap() {
            let parsed = parse_one_measure(&each_result);
            out.insert(parsed.0, parsed.1);
        }
    }
    out
}

fn parse_one_measure(result: &jmespatch::Variable) -> (String, Vec<(String, f64)>) {
    let measure_name = result
        .as_object()
        .unwrap()
        .get("measure_name")
        .unwrap()
        .as_string()
        .unwrap()
        .to_owned();
    let mut data_points = Vec::new();
    for entry in result
        .as_object()
        .unwrap()
        .get("measure_data")
        .unwrap()
        .as_object()
        .unwrap()
    {
        data_points.push((entry.0.to_owned(), entry.1.as_number().unwrap()))
    }
    (measure_name, data_points)
}

pub fn parse_json_pair(
    expression: &jmespatch::Expression<'static>,
    json_response: &str,
) -> Vec<(String, String)> {
    let parsed_json = jmespatch::Variable::from_json(json_response).unwrap();
    let result = expression.search(parsed_json).unwrap();
    debug!("Parsed result: {:?}", result);
    let mut out = Vec::new();
    for entry in result.as_object().unwrap() {
        out.push((entry.0.to_owned(), entry.1.as_string().unwrap().to_owned()))
    }
    out
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_parsing_to_pairs() {
        let json_raw = r#" { "variable_name": "name", "variable_data": "data" } "#;
        let expression =
            jmespatch::compile("{ variable_data: variable_data, variable_name: variable_name }")
                .unwrap();
        let parsed = parse_json_pair(&expression, json_raw);
        let mut reader = HashMap::new();
        for entry in parsed.iter() {
            reader.insert(entry.0.to_owned(), entry.1.to_owned());
        }
        assert_eq!(reader.get("variable_name").unwrap(), "name");
        assert_eq!(reader.get("variable_data").unwrap(), "data");
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
        let datahash = parse_json_data(&expression, &json_raw);
        assert_eq!(
            datahash.get(&String::from("name")).unwrap(),
            &vec!(
                (String::from("desc1"), 1.0 as f64),
                (String::from("desc2"), 2.0 as f64)
            )
        );
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
        let datahash = parse_json_data(&expression, &json_raw);
        assert_eq!(
            datahash.get(&String::from("name1")).unwrap(),
            &vec!(
                (String::from("desc1"), 1.0 as f64),
                (String::from("desc2"), 2.0 as f64)
            )
        );
        assert_eq!(
            datahash.get(&String::from("name2")).unwrap(),
            &vec!(
                (String::from("desc1"), 3.0 as f64),
                (String::from("desc2"), 4.0 as f64)
            )
        );
    }

}
