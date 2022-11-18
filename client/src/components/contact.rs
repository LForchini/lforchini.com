use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div id={"contact"}>
            <h1> {"Contact Me"} </h1>

            <ul>
                <li>
                    <p> {"Mastodon"} </p>
                    <a rel="me" href="https://fosstodon.org/@LForchini"> {"@LForchini@fosstodon.org"} </a>
                </li>

                <li>
                    <p> {"Email"} </p>
                    <a href="mailto: leo@lforchini.com"> {"leo@lforchini.com"} </a>
                </li>
            </ul>
        </div>

    }
}
