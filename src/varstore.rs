use crate::VarPairs;
use percent_encoding::{NON_ALPHANUMERIC, percent_encode};
use crate::SharedVar;
use log::debug;

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
            if start < end {
                let variable = storage_var.read().unwrap();
                if let Some(replaceto) = variable.get(&text[start + 2..end]) {
                    debug!("Using Shared Variable: {:?}", &text[start + 2..end]);
                    newtext = match encode {
                        true => text.replace(
                            &text[start..end + 2],
                            &percent_encode(replaceto.as_bytes(), NON_ALPHANUMERIC).to_string(),
                        ),
                        false => text.replace(&text[start..end + 2], replaceto),
                    };
                } else {
                    newtext = text.replace(&text[start..end + 2], "");
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
pub fn store_variable(
    storage_var: &SharedVar,
    pairs: &VarPairs,
) {
    let mut storage = storage_var.write().unwrap();
    for entry in pairs.iter() {
        storage.insert(entry.0.to_owned(), entry.1.to_owned());
        debug!("Shared Variables added {:?}", entry);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::RwLock;

    #[test]
    fn swap_one_variable() {
        let text_raw = "Text string looking to swap [[ONE]] Variable.";
        let storage_var = Arc::new(RwLock::new(HashMap::new()));
        {
            let mut w = storage_var.write().unwrap();
            w.insert(String::from("ONE"), String::from("1"));
        }
        assert_eq!(
            String::from("Text string looking to swap 1 Variable."),
            swap_variable(&storage_var, text_raw, false)
        );
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
        assert_eq!(
            String::from("Swap 1 Variable and 2 Variables and no Variable."),
            new_text
        );
    }

    #[test]
    fn pairs_into_shared_variables() {
        let storage_var = Arc::new(RwLock::new(HashMap::new()));
        let pairs = vec!((String::from("variable_name"), String::from("name")), (String::from("variable_data"), String::from("data")));
        store_variable(&storage_var, &pairs);
        {
            let reader = storage_var.read().unwrap();
            assert_eq!(reader.get("variable_name").unwrap(), "name");
            assert_eq!(reader.get("variable_data").unwrap(), "data");
        }

    }

}