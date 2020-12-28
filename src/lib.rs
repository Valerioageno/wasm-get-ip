mod utils;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Serialize, Deserialize)]
pub struct IP {
    pub ip: String
}

#[wasm_bindgen]
extern "C" {
    //initialize console.log js function really useful in development mode
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

#[wasm_bindgen]
pub async fn fetch(url: Option<String>) ->Result<JsValue, JsValue> {

    //utils::set_panic_hook();

    //Initialize the http request
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    //Check if url is set or use the default endpoint
    let endpoint: &str = match &url {
        Some(url) => &url[..],
        None => "https://api.ipify.org?format=json"
    };
    
    //Send the request
    let request = Request::new_with_str_and_init(&endpoint, &opts)?;

    //Get the DOM window object
    let window = web_sys::window().unwrap();

    //Get the server response value
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    //Check the validity of the response
    assert!(resp_value.is_instance_of::<Response>());

    //Serialize the json response datatype
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    let ip: IP = json.into_serde().unwrap();

    //Get the DOM document object
    let document = window.document().expect("should have a document on window");

    //Get all DOM nodes that have class ".ipAddress"
    let elements = document.query_selector_all(".ipAddress")
                        .unwrap()
                        .dyn_into::<web_sys::NodeList>()
                        .unwrap();

    //Iterate all elements and inject the ip value in the correct attribute
    for i in 0..elements.length() {

        let elem = elements.item(i).unwrap();

        //log(elem.node_name());

        match &elem.node_name()[..] {
            "INPUT" => elem.dyn_into::<web_sys::HtmlInputElement>().unwrap().set_value(&ip.ip),
            "TEXTAREA" => elem.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap().set_inner_html(&ip.ip),
            _ => elem.dyn_into::<web_sys::HtmlElement>().unwrap().set_inner_html(&ip.ip)
        };
        
    }

    //Return the ip object to the client
    Ok(JsValue::from_serde(&ip).unwrap())
    
}