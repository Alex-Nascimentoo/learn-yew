use yew::prelude::*;
use wasm_bindgen_futures;
use reqwasm::http::Request;
use web_sys::{console, wasm_bindgen::JsValue};
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
  let data = use_state(|| None);
  let cloned_data = data.clone();

  wasm_bindgen_futures::spawn_local(async move {
    let result: Vec<Vehicle> = Request::get("http://localhost:7001/api/vehicle")
      .send()
      .await
      .unwrap()
      .json()
      .await
      .unwrap();

    data.set(Some(result.clone()));
  });

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
        match cloned_data.as_ref() {
          Some(d) => {
            serde_json::to_string(&d).unwrap()
          },
          None => "No data found".to_string()
        }
      }
    </main>
  }
  }