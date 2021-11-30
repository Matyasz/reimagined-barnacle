use yew_router::Switch;

#[derive(Switch, Debug, Clone, Copy)]
pub enum AppRoute {
    #[to = "/hi"]
    Hi,

    #[to = "/"]
    Home,
}
