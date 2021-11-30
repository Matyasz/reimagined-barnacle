pub mod client;
pub mod server;

use client::app::app::AppContainer;

fn main() {
    yew::start_app::<AppContainer>();
}
