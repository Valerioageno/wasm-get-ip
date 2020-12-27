mod utils;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize)]
pub struct IP {
    pub ip: String
}


#[wasm_bindgen]
pub async fn fetch(url: Option<String>) ->Result<JsValue, JsValue> {

    utils::set_panic_hook();

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    // let endpoint: &str = if url == "" {
    //     "https://api.ipify.org?format=json"
    // } else {
    //     &url[..]
    // };

    let endpoint: &str = match &url {
        Some(url) => &url[..],
        None => "https://api.ipify.org?format=json"
    };
    
    let request = Request::new_with_str_and_init(&endpoint, &opts)?;

    let window = web_sys::window().unwrap();
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from rust");

    body.append_child(&val)?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Use serde to parse the JSON into a struct.
    let ip: IP = json.into_serde().unwrap();

    // Send the `Branch` struct back to JS as an `Object`.
    Ok(JsValue::from_serde(&ip).unwrap())
    
}