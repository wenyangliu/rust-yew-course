use std::ops::Deref;

use gloo::console::log;
use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use yew::ContextProvider;
use yew_router::prelude::*;
mod components;
mod router;
use components::atoms::main_title::{Color, MainTitle};
use components::atoms::struct_hello::StructHello;
use components::molecules::custom_form::{CustomForm, Data};
use router::{switch, Route};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct User {
  pub username: String,
  pub favorite_language: String,
}


#[styled_component(App)]
// pub fn app() -> Html {
//   let user_state = use_state(User::default);
//   let main_title_load = Callback::from(|message: String| log!(message));

//   let custom_form_submit = {
//     let user_state = user_state.clone();
//     Callback::from(move |data: Data| {
//       let mut user = user_state.deref().clone();
//       user.username = data.username;
//       user.favorite_language = data.favorite_language;
//       user_state.set(user);
//     })
//   };

//   html! {
//     <ContextProvider<User> context={user_state.deref().clone()}>
//       <MainTitle title="Hi there!" color={Color::Ok} on_load={main_title_load}/>
//       <CustomForm onsubmit={custom_form_submit} />
//       <BrowserRouter>
//         <Switch<Route> render={Switch::render(switch)} />
//       </BrowserRouter>
//     </ContextProvider<User>>
//   }
// }

pub fn app() -> Html {
  html! {
    <div>
      <StructHello message={"Hello from lib.rs".to_owned()} />
    </div>
  }
}