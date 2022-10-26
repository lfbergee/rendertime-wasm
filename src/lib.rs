#![recursion_limit = "256"]

use serde_derive::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement as InputElement;
use yew::html;
use yew::prelude::*;

mod todo;

struct TodoApp {
    todos: Vec<Todo>,
    next_todo: String,
}

enum Msg {
    MakeReq,
    Update(String),
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub title: String,
}

impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let todos = self.todos.clone();
        let cb = ctx.link().callback(|_| Msg::MakeReq);
        let next_todo = self.next_todo.clone();

        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: InputElement = e.target_unchecked_into();
            Msg::Update(input.value())
        });

        html! {
            <div>
                <h1>{"Client side rendering with vite and preact"}</h1>
              <main>
                  <section>
                      <div>
                          <todo::list::List todos={todos.clone()}/>
                      </div>
                      <input
                          type="text"
                          value={next_todo.clone()}
                          oninput={oninput}
                      />
                      <button onclick={cb.clone()}>
                          { "Add" }
                      </button>
                  </section>
              </main>
            </div>
          }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<TodoApp>();
}
