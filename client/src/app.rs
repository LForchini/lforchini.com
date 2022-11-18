use crate::components::mastodon::Mastodon;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
             <h1>{ "Welcome to my web site :)" }</h1>

             <Mastodon />
        </>
    }
}
