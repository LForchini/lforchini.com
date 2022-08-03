use yew::prelude::*;

const PROJECTS: [ProjectInfo; 2] = [
    ProjectInfo {
        name: "DinoDungeons",
        link: Some("http://github.com/LForchini/DinoDungeons"),
        desc: "A small game I made in a team of 4 during my apprenticeship in a 2 day hackathon.",
    },
    ProjectInfo {
        name: "lforchini.com",
        link: Some("http://github.com/LForchini/lforchini.com"),
        desc: "This personal website made entirely in Rust.",
    },
];

struct ProjectInfo {
    name: &'static str,
    link: Option<&'static str>,
    desc: &'static str,
}

pub enum Msg {}

pub struct Projects {}

impl Component for Projects {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id={"projects"}>
                <p> {"Some of the projects I've worked on"} </p>
                <ul>
                    {
                        PROJECTS.into_iter().map(|info| {
                            html!{
                                <li key={info.name.to_string()}>
                                    <p>
                                        { info.name.to_string() }
                                        {
                                            if info.link != None {
                                                html! {
                                                    <>
                                                        { " [" }
                                                        <a href={info.link.unwrap()}>
                                                            { "link" }
                                                        </a>
                                                        { "]" }
                                                    </>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                        { " - " }
                                        { info.desc.to_string() }
                                    </p>
                                </li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            </div>
        }
    }
}
