mod store;
mod display;
mod login;

use store::{YewduxStore, init};
use yew::prelude::*;
use yewdux::prelude::*;
use login:: Login;
use display::Display;

#[function_component(App)]
pub fn view() -> Html {
  html! {
    <div>
      <h1>{"App"}</h1>
      <Login />
      <Display />
    </div>
  }
}
