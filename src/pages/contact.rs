use yew::prelude::*;
use wasm_bindgen_futures;
// use reqwasm::http::Request;
use web_sys::{Request, RequestInit, console, wasm_bindgen::JsValue};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectId {
  oid: String
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Vehicle {
  pub id: Option<ObjectId>,
  pub brand: String,
  pub model: String,
  pub color: String,
  pub price: String,
  wheels: u8,
  doors: u8,
  is_available: bool
}

#[function_component]
pub fn ContactPage() -> Html {
  // let data = use_state(|| None);
  // let cloned_data = data.clone();  
  
  async {
    // async fn fetch_data() -> Result<(), crate::pages::contact::JsValue> {
    //   let mut opts = RequestInit::new();
    //   opts.method("GET");
    //   let request = Request::new_with_str_and_init("http://localhost:7001/api/vehicle", &opts)?;
    //   let window = web_sys::window().unwrap();
    //   wasm_bindgen_futures::JsFuture::from(
    //     window.fetch_with_request(&request)
    //   ).await?;

    //   Ok(())
    // }

    // let _ = fetch_data().await;

    let result = reqwest::get("http://localhost:7001/api/vehicle")
      .await?
      .text()
      .await?;

    console::log_1(&JsValue::from(&result));
  };

  // wasm_bindgen_futures::spawn_local(async move {
  //   let result: Result<Vec<Vehicle>, _> = gloo_net::http::Request::get("http://localhost:7001/api/vehicle")  
  //     .send()
  //     .await
  //     .unwrap()
  //     .json()
  //     .await;
  //     // .unwrap();

  //     console::log_1(&JsValue::from("smt"));

  //   // data.set(Some(result.clone()));
  // });

  html! {
    <main class="container mx-auto">
      <h1
        class="text-5xl text-gray-300 my-5 font-bold"
      >{ "This is the real contact page" }</h1>

      <a
        href="/"
        class="bg-gray-500 px-2 py-1 rounded-md text-xl"
      >{ "Home page" }</a>

      {
        // match cloned_data.as_ref() {
        //   Some(d) => {
        //     // serde_json::to_string(&d).unwrap()
        //     String::from("This is some example data")
        //   },
        //   None => "No data found".to_string()
        // }
        "Smt"
      }
    </main>
  }
  }