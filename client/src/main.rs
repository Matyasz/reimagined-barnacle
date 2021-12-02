pub mod app;
pub mod components;
pub mod pages;
pub mod routes;

use app::app::AppContainer;

fn main() {
    yew::start_app::<AppContainer>();
}
