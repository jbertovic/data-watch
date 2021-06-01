use crate::actors::producer::ApiRequestType;
use http_types::mime;
use log::debug;

/// function makes an api request based on configuration
pub async fn request_api(
    request_type: &ApiRequestType,
    api_url: &str,
    body: &str,
    header: Option<(&str, String)>,
) -> String {
    let mut request = match request_type {
        ApiRequestType::GET => surf::get(api_url),
        ApiRequestType::POST => surf::post(api_url).body(body).content_type(mime::FORM),
    };

    // set header
    if let Some((key, value)) = header {
        request = request.header(key, value);
    };

    let response = request.recv_string().await.unwrap();

    debug!("Response received: {:?}", &response);

    response
}
