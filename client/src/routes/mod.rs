use yew_router::prelude::*;

#[derive(Routable, PartialEq, Debug, Clone, Copy)]
pub enum AppRoute {
    #[at("/login")]
    Login,

    #[at("/signup")]
    Signup,

    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,
}
