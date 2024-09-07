use crate::app::dashboard::Dashboard;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_main(routes: MainRoute) -> Html {
    match routes {
        MainRoute::Home => html! { <Dashboard />  },
        MainRoute::NotFound => html! { "404" },
    }
}
