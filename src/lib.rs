mod store;
mod display;
mod login;
mod api;

use login::Login;
use gloo::console::log;
use serde::{Serialize, Deserialize};
use yewdux::prelude::*;
use std::ops::Deref;
use yew::prelude::*;
use reqwasm::http::Request;
use yewdux_functional::use_store;
use crate::store::YewduxStore;

#[derive(Clone)]
pub struct User {
  pub tasks: Option<Vec<Task>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
  pub completed_at: Option<String>,
  pub description: Option<String>,
  pub id: u32,
  pub priority: char,
  pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskResponse {
  pub data: Vec<Task>,
}

#[function_component(App)]
pub fn view() -> Html {
  let store = use_store::<PersistentStore<YewduxStore>>();
  let state = use_state(|| User {
    tasks: None,
  });
  let onclick = {
    let state = state.clone();
    Callback::from(move |_| {
        let state = state.clone();
        let token = if let Some(state) = store.state() {
          state.token.clone()
        } else {
          String::new()
        };
        wasm_bindgen_futures::spawn_local(async move {
            let response = Request::get("http://127.0.0.1:3000/api/v1/tasks")
                .header("x-auth-token", &token)
                .send()
                .await
                .unwrap()
                .json::<TaskResponse>()
                .await
                .unwrap();
            // log!(serde_json::to_string_pretty(&response).unwrap());
            let mut user = state.deref().clone();
            user.tasks = Some(response.data);
            state.set(user);
        });
    })
  };
  html! {
    <div>
      <h1>{"App"}</h1>
      <Login />
      <button {onclick}>{"get tasks"}</button>
      <pre>{serde_json::to_string_pretty(&state.tasks).unwrap()}</pre>
    </div>
  }
}
