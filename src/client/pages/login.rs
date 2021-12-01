use crate::client::components::navbar::NavBar;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub enum LoginMsg {}

pub struct LoginPage {
    link: ComponentLink<Self>,
}

impl Component for LoginPage {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <NavBar/>

                <div class="credentials-box">
                    { "Log in" }
                </div>
            </div>
        }
    }
}
