use gloo::console::log;
use yew::prelude::*;
use stylist::{yew::styled_component, Style};

mod components;

use components::atoms::main_title::{Color, MainTitle};
use components::molecules::custom_form::{CustomForm, Data};


#[styled_component(App)]
pub fn app() -> Html {
  let main_title_load = Callback::from(|message: String| log!(message));
  let custom_form_submit = Callback::from(|data: Data| {
    log!("username is", data.username);
    log!("favorite_language is", data.favorite_language);
  });
  html! {
    <div>
      <MainTitle title="Hi there!" color={Color::Ok} on_load={main_title_load}/>
      <CustomForm onsubmit={custom_form_submit} />
    </div>
  }
}