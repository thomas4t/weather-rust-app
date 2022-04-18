use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod forecast;
pub mod home;

use about::About;
use forecast::Forecast;
use home::Home;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/forecast")]
    Forecast,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
}

/// Switch app routes
pub fn switch(routes: &AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Home => html! { <Home /> },
        AppRoute::Forecast => html! { <Forecast /> },
        AppRoute::About => html! { <About /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}
