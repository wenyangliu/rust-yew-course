mod counter;
mod store;
mod display;
mod router;

use store::{YewduxStore, init};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::{prelude::*, dispatch};
use counter:: Counter;
use display::DisplayCount;
use router::{switch, Route};

pub struct App {
  dispatch: Dispatch<BasicStore<YewduxStore>>
}

impl Component for App {
    type Message = ();

    type Properties = DispatchProps<BasicStore<YewduxStore>>;

    fn create(ctx: &Context<Self>) -> Self {
      let dispatch = init();
      Self {dispatch}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
      html! {
        <div>
          <h1>{"App"}</h1>
          <WithDispatch<Counter> />
          <WithDispatch<DisplayCount> />
          <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
          </BrowserRouter>
        </div>
      }
    }
}