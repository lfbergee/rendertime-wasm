use super::super::Todo;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub todos: Vec<Todo>,
}

pub struct List;

impl List {
    fn render_list(&self, todos: &Vec<Todo>) -> Html {
        html! {
                <ul>
                    { todos.iter().map(|todo| self.view_todo(todo)).collect::<Html>() }
                </ul>
        }
    }
    fn view_todo(&self, todo: &Todo) -> Html {
        html! {
            <li>
                <label>
                    <input type="checkbox"  />
                    <span class={classes!("checkmark")}></span>
                    { &todo.title }
                </label>
            </li>
        }
    }
}

pub enum Msg {}

impl Component for List {
    type Properties = Props;
    type Message = Msg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { self.render_list(&ctx.props().todos)}
            </div>
        }
    }
}
