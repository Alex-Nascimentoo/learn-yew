use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::home::Home;
use crate::pages::contact::Contact;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/contact")]
  Contact
}

pub fn switch(routes: Route) -> Html {
  match routes {
    // Route::Home => html! { <a href="/contact">{ "This is the home page" }</a> },
    Route::Home => html! { <Home /> },
    Route::Contact => html! { <Contact /> },
  }
}