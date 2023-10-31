use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/contact")]
  Contact
}

pub fn switch(routes: Route) -> Html {
  match routes {
    Route::Home => html! { <a href="/contact">{ "This is the home page" }</a> },
    Route::Contact => html! { <h1>{ "This is the contact page" }</h1> },
  }
}