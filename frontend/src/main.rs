//use yew::prelude::*;
use yew::{classes, html, Component, Context, Html};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

pub enum Msg {
    Add,
    Remove,
    Home,
    Details,
//    Login,
//    LogOut,
}

pub struct App {
  projects: Vec<Html>,
}

impl App {
    pub fn add(&mut self) {

    }

    pub fn delete(& mut self) {

    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
           projects: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                self.projects.push(html! {
                    <>
                    <div>
                    <h2>{"Hello!"}</h2>
                    </div>
                    </>
                });
                true
            }
            Msg::Remove => {
                true
            }
            Msg::Home => {true}
            Msg::Details => {true},
            _ => {true},
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"/>
            <div class="topnav">
              <a class="active" href="#home">{"Chaos"}</a>
              <div id={"myLinks"}>
                <a href="#add">{"Add"}</a>
                <a href="#review">{"Review"}</a>
              </div>
            </div>
            <div class="option-buttons">
              <button class="option-button" onclick={ctx.link().callback(|_| Msg::Add)}>{"Add"}</button>
              <button class="option-button" onclick={ctx.link().callback(|_| Msg::Remove)}>{"Remove"}</button>
            </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
