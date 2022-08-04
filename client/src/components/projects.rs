use shared::Project;
use yew::prelude::*;

pub enum Msg {}

#[derive(Clone)]
pub struct Projects {
    projects: Vec<Project>,
}

impl Component for Projects {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let projects = vec![];

        Self { projects }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id={"projects"}>
                <p> {"Some of the projects I've worked on"} </p>

                {
                    match self.projects.len() {
                        0 => {
                            html! {
                                <p>
                                    { "My projects can't be loaded right now, sorry" }
                                </p>
                            }
                        }
                        _ => {
                            html!{
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
                        }
                        }
                    }

                }
            </div>
        }
    }
}
