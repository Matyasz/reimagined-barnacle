use crate::components::signup_form::SignupForm;

use yew::{html, Component, Context, Html};

pub enum SignupMsg {}

pub struct SignupPage {}

impl Component for SignupPage {
    type Message = SignupMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <SignupForm />
            </div>
        }
    }
}
