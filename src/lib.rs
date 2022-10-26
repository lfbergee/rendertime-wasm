#![recursion_limit = "256"]

use serde_derive::Deserialize;
use wasm_bindgen::prelude::*;
use yew::html;
use yew::prelude::*;
use yew_router::{components::RouterAnchor, router::Router, Switch};

mod todo;

pub type Anchor = RouterAnchor<AppRoute>;

struct TodoApp {
    link: ComponentLink<Self>,
    todos: Vec<Todo>,
    next_todo: String,
}

enum Msg {
    MakeReq,
    Update(String),
}

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/"]
    Home,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub title: String,
}

impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq);
        Self {
            link,
            todos: vec![
                {
                    Todo {
                        title: "Write talk for dev/øl".to_string(),
                    }
                },
                {
                    Todo {
                        title: "Create live examples".to_string(),
                    }
                },
                {
                    Todo {
                        title: "Run demo at dev/øl".to_string(),
                    }
                },
            ],
            next_todo: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                if !self.next_todo.is_empty() {
                    self.todos.push({
                        Todo {
                            title: self.next_todo.clone(),
                        }
                    });
                    self.next_todo = "".to_string();
                }
                ()
            }
            Msg::Update(next_todo) => {
                self.next_todo = next_todo;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let todos = self.todos.clone();
        let cb = self.link.callback(|_| Msg::MakeReq);
        let on_input_change = self
            .link
            .callback(|event: InputData| Msg::Update(event.value));
        let next_todo = self.next_todo.clone();

        html! {
            <main>
                <section>
                    <Router<AppRoute, ()>
                        render = Router::render(move |switch: AppRoute| {
                            match switch {
                                AppRoute::Home => {
                                    html! {
                                        <div>
                                            <todo::list::List todos=todos.clone()/>
                                        </div>
                                    }
                                }
                            }
                        })
                    />
                    <input
                        type="text"
                        oninput=on_input_change.clone()
                        value=next_todo.clone()
                    />
                    <button onclick=cb.clone()>
                        { "Add" }
                    </button>
                </section>
            </main>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<TodoApp>::new().mount_to_body();
}
