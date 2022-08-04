use shared::Project;
use yew::prelude::*;

pub enum Msg {}

pub struct Projects {
    projects: Vec<Project>,
}

impl Component for Projects {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let projects = vec![Project {
        name: "DinoDungeons".to_string(),
        link: Some("http://github.com/LForchini/DinoDungeons".to_string()),
        desc: "A small game I made in a team of 4 during my apprenticeship in a 2 day hackathon.".to_string(),
    },
    Project{
        name: "lforchini.com".to_string(),
        link: Some("http://github.com/LForchini/lforchini.com".to_string()),
        desc: "This personal website made entirely in Rust.".to_string(),
    }];

        Self { projects }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id={"projects"}>
                <p> {"Some of the projects I've worked on"} </p>
                <ul>
                    {
                        self.projects.clone().into_iter().map(|info| {
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
