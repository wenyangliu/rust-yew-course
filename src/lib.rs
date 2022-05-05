use yew::prelude::*;
use stylist::{yew::styled_component, Style};

mod components;

use components::atoms::main_title::{Color, MainTitle};

const STYLE_FILE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
  let stylesheet = Style::new(STYLE_FILE).unwrap();
  html! {
    <div class={stylesheet}>
      <MainTitle title="Hi there!" color={Color::Ok} />
    </div>
  }
}