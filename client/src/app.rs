use crate::components::projects;
use yew::prelude::*;

pub enum Msg {}

pub struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div id={"intro"}>
                    <p> {"I'm "}
                    <a href={"http://github.com/LForchini"}> {"Leonardo Forchini"} </a>
                    {", and I'm interested in most parts of computer science."} </p>
                </div>

                <div id={"contact"}>
                    <p> {"You can contact me with the methods below"} </p>
                    <ul>
                        <li>
                            <p> {"Email: leonardo.forchini@gmail.com"} </p>
                        </li>
                    </ul>
                </div>

                <projects::Projects />
            </>
        }
    }
}
