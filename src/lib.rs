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
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

#[wasm_bindgen]
pub async fn fetch(url: Option<String>) ->Result<JsValue, JsValue> {

    //utils::set_panic_hook();

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let endpoint: &str = match &url {
        Some(url) => &url[..],
        None => "https://api.ipify.org?format=json"
    };
    
    let request = Request::new_with_str_and_init(&endpoint, &opts)?;

    let window = web_sys::window().unwrap();

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();

    
    let json = JsFuture::from(resp.json()?).await?;

    let ip: IP = json.into_serde().unwrap();

    let document = window.document().expect("should have a document on window");

    let elements = document.query_selector_all(".ipAddress")
                        .unwrap()
                        .dyn_into::<web_sys::NodeList>()
                        .unwrap();

    for i in 0..elements.length() {

        let elem = elements.item(i).unwrap();

        //log(elem.node_name());

        if  elem.node_name() == "INPUT" {
            
            elem.dyn_into::<web_sys::HtmlInputElement>().unwrap().set_value(&ip.ip);

        }else if elem.node_name() == "TEXTAREA" {

            elem.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap().set_inner_html(&ip.ip);

        }else {

            elem.dyn_into::<web_sys::HtmlElement>().unwrap().set_inner_html(&ip.ip);
        }
        
    }

    Ok(JsValue::from_serde(&ip).unwrap())
    
}