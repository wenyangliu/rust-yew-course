use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::pages::home::Home;
use crate::components::pages::hello::Hello;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello")]
    Hello,
}
pub fn switch(route: &Route) -> Html {
  match route {
      Route::Home => html! {<Home />},
      Route::Hello => html! {<Hello />},
  }
}