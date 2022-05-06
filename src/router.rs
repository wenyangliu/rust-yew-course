use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use super::counter::Counter;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Counter,
}
pub fn switch(route: &Route) -> Html {
  match route {
      Route::Counter => html! {<WithDispatch<Counter> />},
  }
}