use yew::prelude::*;

pub struct Home;

impl Component for Home {
  type Message = ();

  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <main class="container mx-auto">
        <h1>{ "This is the original home page!" }</h1>
        <a
          href="/contact"
          class="bg-gray-500 px-2 py-1 rounded-md text-xl"
        >
          { "Contact page" }
        </a>
      </main>
    }
  }
}