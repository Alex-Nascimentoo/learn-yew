use yew::prelude::*;

struct Counter {
  value: i16
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub text: String
}

#[function_component]
pub fn Component(props: &Props) -> Html {
  let count = use_state(|| Counter { value: 0 });

  let count_plus = {
    let count = count.clone();

    Callback::from(move |_| {
      count.set(Counter { value: count.value + 1 })
    })
  };

  let count_minus = {
    let count = count.clone();

    Callback::from(move |_| {
      count.set(Counter { value: count.value - 1 })
    })
  };

  html! {
    <>
      <h1>{ &props.text }</h1>
      <button onclick={count_minus} >{ "minus" }</button>
      <h1>{ "The value is: " } { count.value }</h1>
      <button onclick={count_plus} >{ "plus" }</button>    
    </>
  }
}

#[function_component]
pub fn Home() -> Html {
  html! {
    <h1>{ "This is the home page" }</h1>
  }
}
