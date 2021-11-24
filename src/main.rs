pub mod server;
pub mod ui;

use ui::components::navbar::NavBar;

fn main() {
    yew::start_app::<NavBar>();
    // yew::start_app::<NavBar>();
}
