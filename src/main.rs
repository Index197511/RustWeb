#[macro_use]
extern crate yew;
use yew::prelude::{App, Component, ComponentLink, Html, Renderable, ShouldRender};

struct Model {
    count: i32,
}

impl Model{
    fn count_up(&mut self) -> () {
        self.count += 1;
    }

    fn count_down(&mut self) -> () {
        self.count -= 1;
    }
}

enum Msg {
    Increment,
    Decrement,
}

impl Component for Model{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self{
        Model{
            count: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender{
        match msg{
            Msg::Increment => {
                self.count_up();
                true
            }

            Msg::Decrement => {
                self.count_down();
                true
            }
        }
    }
}

impl Renderable<Model> for Model{
    fn view(&self) -> Html<Self>{
        html! {
            <div>
                <p>{self.count}</p>

                <div>
                    <button onclick = |_| Msg::Increment,> {"Increment"} </button>
                </div>

                <div>
                    <button onclick = |_| Msg::Decrement,> {"Decrement"} </button>
                </div>

            </div>
        }
    }
}

fn main(){
    yew::start_app::<Model>();
}