use yew::prelude::*;

pub struct Contact;

impl Component for Contact {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <main class="container mx-auto">
        <h1>{ "This is the real contact page" }</h1>

        <a
          href="/"
          class="bg-gray-500 px-2 py-1 rounded-md text-xl"
        >{ "Home page" }</a>
      </main>
    }
  }
}