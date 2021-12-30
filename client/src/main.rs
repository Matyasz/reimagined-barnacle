pub mod app;
pub mod components;
pub mod pages;
pub mod routes;

use app::app::AppContainer;
// use wasm_bindgen::prelude::wasm_bindgen;

fn main() {
    yew::start_app::<AppContainer>();
}

// #[wasm_bindgen(start)]
// pub fn run_app() {
//     App::<AppContainer>::new().mount_to_body();
// }
