use gloo::console::log;
use yew::prelude::*;
use stylist::{yew::styled_component, Style};

mod components;

use components::atoms::main_title::{Color, MainTitle};
use components::molecules::custom_form::CustomForm;


#[styled_component(App)]
pub fn app() -> Html {
  let main_title_load = Callback::from(|message: String| log!(message));
  html! {
    <div>
      <MainTitle title="Hi there!" color={Color::Ok} on_load={main_title_load}/>
      <CustomForm />
    </div>
  }
}