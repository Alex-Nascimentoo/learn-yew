use yew::prelude::*;
use yew_router::prelude::*;

mod router;
mod pages;

// use component::Component;
use router::*;

#[function_component(App)]
fn app() -> Html {
  html! {
    <BrowserRouter>
      <Switch<Route> render={switch} />
    </BrowserRouter>

    // <>
    //   <Component text="Something else" />
    // </>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
