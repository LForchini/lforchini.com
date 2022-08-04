use shared::Project;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use yew::prelude::*;

pub enum Msg {
    SetProjects(Vec<Project>),
    FetchProjects,
    None,
}

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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetProjects(projects) => {
                self.projects = projects;
                true
            }
            Msg::FetchProjects => {
                ctx.link().send_future(async {
                    match Self::fetch_projects().await {
                        Ok(projects) => Msg::SetProjects(projects),
                        Err(_) => Msg::None,
                    }
                });
                false
            }
            Msg::None => false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::FetchProjects);
        }
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

impl Projects {
    async fn fetch_projects() -> Result<Vec<Project>, FetchError> {
        let mut opts = RequestInit::new();
        opts.method("GET");

        let mut url = "/api/projects";
        let window = gloo_utils::window();
        let location = window.location().host()?;
        if location.split(':').collect::<Vec<&str>>()[0] == "localhost" {
            url = "http://localhost:8088/projects"
        }
        log::info!("trying to fetch from {:#?}", url);
        let request = Request::new_with_str_and_init(url, &opts)?;

        let response = JsFuture::from(window.fetch_with_request(&request)).await?;
        let response: Response = response.dyn_into().unwrap();

        let projects_string = JsFuture::from(response.text()?).await?.as_string().unwrap();

        match serde_json::from_str(&projects_string) {
            Ok(projects) => Ok(projects),
            Err(_) => Err(FetchError {
                err: JsValue::from_str("Parsing error"),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.err, f)
    }
}
impl std::error::Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}
