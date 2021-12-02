use crate::components::{navbar::NavBar, signup_form::SignupForm};

use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub enum SignupMsg {}

pub struct SignupPage {
    link: ComponentLink<Self>,
}

impl Component for SignupPage {
    type Message = SignupMsg;
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
                <NavBar />
                <SignupForm />
            </div>
        }
    }
}
