use crate::components::contact::Contact;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
             <h1>{ "Welcome to my web site :^) -- Under Construction" }</h1>

             <footer>
                <Contact />
             </footer>

        </>
    }
}
