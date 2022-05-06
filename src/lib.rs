mod store;
mod display;
mod login;

use store::{YewduxStore, init};
use yew::prelude::*;
use yewdux::prelude::*;
use login:: Login;
use display::DisplayForm;

pub struct App {
  _dispatch: Dispatch<BasicStore<YewduxStore>>
}

impl Component for App {
    type Message = ();

    type Properties = DispatchProps<BasicStore<YewduxStore>>;

    fn create(_ctx: &Context<Self>) -> Self {
      let _dispatch = init();
      Self {_dispatch}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
      html! {
        <div>
          <h1>{"App"}</h1>
          <WithDispatch<Login> />
          <WithDispatch<DisplayForm> />
        </div>
      }
    }
}