use yew_router::Switch;

#[derive(Switch, Debug, Clone, Copy)]
pub enum AppRoute {
    #[to = "/login"]
    Login,

    #[to = "/signup"]
    Signup,

    #[to = "/"]
    Home,
}
