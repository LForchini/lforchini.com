use yew::prelude::*;

struct ContactItem {
    service: String,
    href: String,
    disp: String,
}

#[function_component(Contact)]
pub fn contact() -> Html {
    let contacts = vec![
        ContactItem {
            service: "Mastodon".into(),
            href: "https://fosstodon.org/@LForchini".into(),
            disp: "@LForchini@fosstodon.org".into(),
        },
        ContactItem {
            service: "Email".into(),
            href: "mailto:leo@lforchini.com".into(),
            disp: "leo@lforchini.com".into(),
        },
    ];

    html! {
        <div id={"contact"}>
            <h2> {"Contact Me"} </h2>

            <ul>
                {
                    contacts.into_iter().map(|contact| {
                        html! {
                            <li key={contact.service.clone()}>
                                <a href={contact.href}> {contact.service + ": " + &contact.disp} </a>
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </div>

    }
}
