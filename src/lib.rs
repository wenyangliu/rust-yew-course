use yew::prelude::*;
use stylist::{yew::styled_component, style};

#[styled_component(App)]

pub fn app() -> Html {
  let stylesheet = style!(
    r#"
      h1 {
        color: orange;
      }
      p {
        color: green;
      }
    "#
  ).unwrap();
  html! {
    <div class={stylesheet}>
      <h1>{"Hello World!!!"}</h1>
      <p>{"more text"}</p>
    </div>
  }
}